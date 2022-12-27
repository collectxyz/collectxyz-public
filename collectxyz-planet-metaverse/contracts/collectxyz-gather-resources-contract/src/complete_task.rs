use std::collections::HashMap;
use serde_json;

use collectxyz_resources::CollectXyzResourceExecuteMsg;
use cosmwasm_std::{Addr, BlockInfo, QuerierWrapper, Response, StdError, StdResult, Storage, Uint128, WasmMsg, to_binary};
use fixed::types::I20F12;
use rust_decimal::prelude::ToPrimitive;

use collectxyz_planet_metaverse::{discover_planets::PlanetCoordinates, randomness_msg, util::{fetch_nft_data, fetch_random_numbers, validate_nft_is_owned_by_wallet}};

use crate::{state::{CONFIG, NFT_ID_GATHERING_RESOURCES, ResourceGatherInfo, TASK_REPOSITORY, load_all_resource_gathering_info}};

/// Generates the yield for this resource harvest task
///
/// Using the params on [ResourceGatherInfo], the base yield
/// is generated and some deviation is factored in randomly using
/// the given numbers.
fn generate_yield(
    richness_score: u8,
    resource_gather_info: &ResourceGatherInfo,
    random_number_deviation_magnitude: u8,
    random_number_deviation_direction: u8,
) -> u64 {
    let base_yield = richness_score.to_u64().unwrap() * resource_gather_info.base_yield;
    let deviation = I20F12::from_num(resource_gather_info.max_deviation_yield) * richness_score.to_i32().unwrap() * random_number_deviation_magnitude.to_i32().unwrap() / u8::MAX.to_i32().unwrap() ;
    if random_number_deviation_direction < resource_gather_info.deviation_direction_threshhold {
        return base_yield - deviation.floor().to_num::<u64>();
    }
    return base_yield + deviation.floor().to_num::<u64>();
}

/// Mints the yield amount worth of resource
pub fn mint_yield_as_resources(
    resource_gather_infos: &HashMap<String, ResourceGatherInfo>,
    resource_id: &String, 
    total_yield: Uint128,
    recipient_xyz_id: &String,
) -> StdResult<WasmMsg> {
    let mint_resources_for_recipient_xyz_id = CollectXyzResourceExecuteMsg::Mint {
        recipient_xyz_id: recipient_xyz_id.to_string(),
        amount: total_yield
    };
    let resource_gather_info = resource_gather_infos.get(&resource_id.to_string());
    if resource_gather_info.is_none() {
        return Err(StdError::generic_err("Could not get data"));
    }
    return Ok(
        WasmMsg::Execute {
            contract_addr: resource_gather_info.unwrap().resource_contract_address.to_string(),
            msg: to_binary(&mint_resources_for_recipient_xyz_id)?,
            funds: vec![],
        }
    );
}

fn mint_bonus_token(
    recipient: &String,
    bonus_token_address: &Addr
) -> StdResult<WasmMsg> {
    let mint_bonus_token_msg = randomness_msg::ExecuteMsg::MintBonusToken {
        recipient: recipient.to_string(),
        amount: Uint128::new(1000000)
    };
    return Ok(
        WasmMsg::Execute {
            contract_addr: bonus_token_address.to_string(),
            msg: to_binary(&mint_bonus_token_msg)?,
            funds: vec![],
        }
    );
}
/// Generates the minimum yield for this resource harvest task
///
/// Using the params on [ResourceGatherInfo], the base yield
/// is generated and some deviation is factored in randomly using
/// the given numbers.
fn generate_min_yield(
    richness_score: u8,
    resource_gather_info: &ResourceGatherInfo,
) -> u64 {
    let base_yield = richness_score.to_u64().unwrap() * resource_gather_info.base_yield;
    let deviation = I20F12::from_num(resource_gather_info.max_deviation_yield) * richness_score.to_i32().unwrap();
    return base_yield - deviation.floor().to_num::<u64>();
}

pub fn complete_task(
    xyz_nft_id: &String,
    claimed_xyz_owner_addr: &String,
    block: &BlockInfo,
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
) -> Result<Response, StdError> {
    let config = CONFIG.load(storage)?;

    let resource_gather_infos: HashMap<String, ResourceGatherInfo> = load_all_resource_gathering_info(storage)?;
    // Fail if the NFT is not owned by sender
    if !validate_nft_is_owned_by_wallet(
        xyz_nft_id,
        claimed_xyz_owner_addr,
        querier,
        &config.xyz_nft_contract_address,
    )? {
        return Err(StdError::generic_err("Wallet does not own NFT"));
    }

    let existing_task = TASK_REPOSITORY.fetch_existing_task(xyz_nft_id, storage)?;

    // Fail if the task is not complete
    if !existing_task.is_task_complete(block) {
        return Err(StdError::generic_err("Task is not completed yet"));
    }

    /****** Coordinate checks ******/
    let nft_info = fetch_nft_data(
        &existing_task.nft_token_id,
        &config.xyz_nft_contract_address,
        querier,
    )?;

    // Fail if xyz is moving
    if !nft_info.extension.has_arrived(block.time) {
        return Err(StdError::generic_err(
            "Cannot complete a task for moving nft.",
        ));
    }

    // No-op nothing if NFT position does not match Task
    if PlanetCoordinates::from_xyz_coordinates(nft_info.extension.coordinates)? != existing_task.coordinates {
        TASK_REPOSITORY.remove_task(storage, &existing_task)?;
        NFT_ID_GATHERING_RESOURCES.remove(storage, &xyz_nft_id);
        return Ok(Response::default()
            .add_attribute("action", "no-op")
            .add_attribute(
                "reason",
                "Task coordinates dont match current xyz coordinates",
            ));
    }

    // At this point the Task is owner is validated and the task is complete
    let mut random_numbers = fetch_random_numbers(
        querier,
        &config.randomness_contract_address,
        config.gather_task_duration_seconds,
        &existing_task.start_time,
        &existing_task.nft_token_id
    )?;

    // map of yields resource_id -> yield total
    let resources = NFT_ID_GATHERING_RESOURCES.load(storage, xyz_nft_id)?;
    let mut resource_yield_map: HashMap<String, Uint128> = HashMap::new();
    for resource in resources.iter()
    {
        let resource_id = &resource.resource_identifier;
        let resource_gather_info = resource_gather_infos.get(resource_id);
        if resource_gather_info.is_none() {
            return Err(StdError::generic_err(format!(
                "No mapping found in config for resource with Id: {:?}",
                resource_id.to_string()
            )));
        }
        let resource_gather_info = resource_gather_info.unwrap();
        let gather_yield: Uint128 = if existing_task.is_task_expired(block) {
            // If the task is expired we will yield the minimum amount of the resources
            Uint128::from(generate_min_yield(
                resource.resource_richness_score, 
                resource_gather_info
            )).checked_mul(Uint128::from(1000000u64)).unwrap_or(Uint128::from(0u64))
        } else {
            let r = random_numbers.pop();
            if r.is_none() {
                return Err(StdError::generic_err(
                    "Not enough random numbers",
                ));    
            }
            let s = random_numbers.pop();
            if s.is_none() {
                return Err(StdError::generic_err(
                    "Not enough random numbers",
                ));    
            }

            Uint128::from(generate_yield(
                resource.resource_richness_score,
                resource_gather_info,
                r.unwrap(),
                s.unwrap(),
            )).checked_mul(Uint128::from(1000000u64)).unwrap_or(Uint128::from(0u64))
        };
        
        let new_cumulative_yield = resource_yield_map
            .get_mut(resource_id)
            .unwrap_or(&mut Uint128::from(0u64))
            .checked_add(gather_yield)
            .unwrap_or(Uint128::MAX);
        resource_yield_map.insert(resource_id.to_string(), new_cumulative_yield);
    }
    let mut messages: Vec<WasmMsg> = vec![];
    let resource_attributes: Vec<(String, String)>= vec![("resources_gathered".to_string(), serde_json::to_string(&resource_yield_map).unwrap())];
    for (resource_id, yield_total) in resource_yield_map.iter() {
        let mint_resource_message = mint_yield_as_resources(
            &resource_gather_infos, 
            resource_id, 
            yield_total.clone(), 
            xyz_nft_id
        )?;
        messages.push(mint_resource_message);
    }

    let r = random_numbers.pop();
    if r.is_none() {
        return Err(StdError::generic_err(
            "Not enough random numbers",
        ));    
    }
    if r.unwrap() < config.bonus_token_probability {
        let mint_bonus_token = mint_bonus_token(
            claimed_xyz_owner_addr,
            &config.randomness_contract_address,
        )?;
        messages.push(mint_bonus_token);
    }
    
    TASK_REPOSITORY.remove_task(storage, &existing_task)?;
    NFT_ID_GATHERING_RESOURCES.remove(storage, &xyz_nft_id);

    let xyz_exp_config = config.experience_mint_config;
    let xyz_exp_msg = xyz_exp_config.mint_experince(
        xyz_nft_id.to_string()
    )?;
    messages.push(xyz_exp_msg);

    return Ok(
        Response::new()
            .add_attribute("method", "complete task")
            .add_attribute("xyz_id", xyz_nft_id.to_string())
            .add_attribute("task_type", "gather resources")
            .add_attribute("experience_gained", xyz_exp_config.complete_task_experience_amount)
            .add_attributes(resource_attributes)
            .add_messages(messages)
    );
}

#[cfg(test)]
mod tests {
    use collectxyz_planet_metaverse::mock_querier::DEFAULT_RAND;

    use crate::{complete_task::{generate_min_yield, generate_yield}, state::ResourceGatherInfo};

    #[test]
    fn test_generate_yield() {
        let resource_gather_info = ResourceGatherInfo {
            resource_identifier: "xyzIRON".to_string(),
            resource_contract_address: "resourceContractAddress".to_string(),
            base_yield: 10,
            max_deviation_yield: 5,
            deviation_direction_threshhold: 128,
        };

        let expected_yield = 9u64; // 10 + floor(-1 * 74/255 * 5) = 9
        let resource_yield = generate_yield(
            1,
            &resource_gather_info,
            DEFAULT_RAND[0], // 74
            DEFAULT_RAND[1], // 105
        );

        println!("{:?}", resource_yield);
        assert_eq!(resource_yield, expected_yield);

        let expected_yield = 33u64; // 30 + floor(3 * 74/255 * 5) = 9
        let resource_yield = generate_yield(
            3,
            &resource_gather_info,
            DEFAULT_RAND[3], // 67
            DEFAULT_RAND[4], // 234
        );

        println!("{:?}", resource_yield);
        assert_eq!(resource_yield, expected_yield);
    }

    #[test]
    fn test_generate_min_yield() {
        let resource_gather_info = ResourceGatherInfo {
            resource_identifier: "xyzIRON".to_string(),
            resource_contract_address: "resourceContractAddress".to_string(),
            base_yield: 10,
            max_deviation_yield: 5,
            deviation_direction_threshhold: 50,
        };

        let expected_yield = 5u64; // 10 - 5
        let resource_yield = generate_min_yield(
            1,
            &resource_gather_info,
        );

        println!("{:?}", resource_yield);
        assert_eq!(resource_yield, expected_yield);
    }
}
