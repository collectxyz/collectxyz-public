use collectxyz_planet_metaverse::discover_planets::PlanetCoordinates;
use cosmwasm_std::{BlockInfo, MessageInfo, QuerierWrapper, Response, StdError, StdResult, Storage};

use crate::planet_util::is_planet_limit_reached;
use crate::state::{CONFIG, Config, TASK_REPOSITORY};

use collectxyz_planet_metaverse::tasks::{Task};
use collectxyz::nft::XyzTokenInfo;

use collectxyz_planet_metaverse::util::{validate_nft_is_owned_by_wallet,fetch_nft_data, burn_bonus_tokens};
use collectxyz_planet_metaverse::util::{check_sufficient_funds};

fn calculate_expected_boost(config: &Config, bonus_token_count: u8) -> u8 {
    return bonus_token_count * config.boost_per_bonus_token;
}

fn attempt_start_task(
    storage: &dyn Storage,
    block: BlockInfo,
    nft_token_id: &String,
    config: &Config,
    bonus_token_count: u8,
    nft_data: XyzTokenInfo
) -> StdResult<Task> {
    let coordinates = &PlanetCoordinates::from_xyz_coordinates(nft_data.extension.coordinates)?;
    // Dont start task if the planet limit is already reached
    if is_planet_limit_reached(
        storage,
        config,
        coordinates
    ) {
        return Err(StdError::generic_err(
            "Max allowed number of planets discovered!",
        ));
    }

    let new_discover_info: Task = Task::new(
        &nft_token_id.to_string(),
        &block.time,
        calculate_expected_boost(config, bonus_token_count),
        coordinates,
        config.required_seconds,
        config.discovery_task_expiration_window_seconds,
    );

    Ok(new_discover_info)
}

pub fn try_start_task(
    xyz_nft_id: String,
    info: MessageInfo,
    bonus_token_count: u8,
    storage: &mut dyn Storage,
    block: BlockInfo,
    querier: &QuerierWrapper,
) -> Result<Response, StdError> {
    let config = CONFIG.load(storage)?;

    let claimed_owner_addr = info.sender.to_string();
    
    // check that task fee is covered
    check_sufficient_funds(info.funds, &config.start_task_fee)?;

    // Cant boost task with more than the allowed bonus token amount
    if bonus_token_count > config.max_number_of_bonus_tokens {
        return Err(StdError::generic_err(
            "More bonus tokens than allowed for a task boost.",
        ));
    }

    // Only the nft owner can start a task. Here we validate that this nft is owned by
    // the sender using the cw721 contract
    if !validate_nft_is_owned_by_wallet(
        &xyz_nft_id,
        &claimed_owner_addr,
        querier,
        &config.xyz_nft_contract_address,
    )? {
        return Err(StdError::generic_err("Wallet does not own NFT"));
    }

    // Cant start a Task for an XYZ nft that is moving
    let nft_data = fetch_nft_data(
        &xyz_nft_id,
        &config.xyz_nft_contract_address,
        querier
    )?;
    if !nft_data.extension.has_arrived(block.time) {
        return Err(StdError::generic_err("Cannot start a task for moving nft."))
    }

    let existing_task: StdResult<Task> = TASK_REPOSITORY.fetch_existing_task(&xyz_nft_id, storage);

    // There is already a task in progress, we should fail here. Only one task at a time
    if existing_task.is_ok() {
        return Err(StdError::generic_err(
            "Existing discover is still in progress",
        ));
    }

    // Create and save the task. This task will have the boost from the bonus token applied.
    let task: Task = attempt_start_task(storage, block, &xyz_nft_id, &config, bonus_token_count, nft_data)?;
    TASK_REPOSITORY.save_task(storage, &task)?;

    // Execute burn on cw20 contract to burn the supplied bonus tokens as they have been applied to the task.
    if bonus_token_count > 0 {
        return burn_bonus_tokens(claimed_owner_addr.to_string(), config.cw20_bonus_token_contract.to_string(), bonus_token_count);
    }

    return Ok(
        Response::default()
            .add_attribute("method", "start task")
            .add_attribute("xyz_id", xyz_nft_id.to_string())
            .add_attribute("task_type", "planet discovery")
    );
}

pub fn query_task_for_nft(
    storage: &dyn Storage,
    xyz_nft_id: &String,
) -> StdResult<Option<Task>> {
    let task_result = TASK_REPOSITORY.fetch_existing_task(xyz_nft_id, storage);
    match task_result {
        Ok(discovery) => Ok(Some(discovery)),
        Err(_) => Ok(None),
    }
}
