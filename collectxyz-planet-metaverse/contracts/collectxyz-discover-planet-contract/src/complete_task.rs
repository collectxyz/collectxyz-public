
use cosmwasm_std::{ BlockInfo, QuerierWrapper, Response, StdError, StdResult, Storage};
use cw2::get_contract_version;
use rust_decimal::prelude::ToPrimitive;
use serde_json;

use crate::msg::{ResourceGenerationInfo, RichnessThreshold};
use crate::planet_util::is_planet_limit_reached;
use crate::state::{CONFIG, Config, TASK_REPOSITORY};
use collectxyz_planet_metaverse::tasks::{Task};
use collectxyz_planet_metaverse::util::{fetch_nft_data, fetch_random_numbers, validate_nft_is_owned_by_wallet};
use collectxyz::nft::{XyzExtension};
use crate::planet_repository::{save_planet};
use collectxyz_planet_metaverse::discover_planets::{Planet, PlanetCoordinates, PlanetResource};

fn generate_richness(random_number: u8, richness_threshholds: &RichnessThreshold) -> u8 {
    if random_number <= richness_threshholds.level_one {
        return 1;
    }

    if random_number <= richness_threshholds.level_two {
        return 2;
    }

    if random_number <= richness_threshholds.level_three {
        return 3;
    }

    if random_number <= richness_threshholds.level_four {
        return 4;
    }

    if random_number <= richness_threshholds.level_five {
        return 5;
    }

    return 1;
}

fn generate_resource_for_planet(
    random_number_appearance: u8,
    random_number_richness: u8,
    resource_generation_info: &ResourceGenerationInfo,
) -> Option<PlanetResource> {
    if random_number_appearance > resource_generation_info.appearance_probability {
        return None;
    }

    let richness: u8 = generate_richness(
        random_number_richness,
        &resource_generation_info.richness_thresholds,
    );

    let desc = PlanetResource {
        resource_identifier: resource_generation_info.resource_identifier.to_string(),
        resource_richness_score: richness,
    };
    return Some(desc);
}

fn generate_planet(
    xyz_nft_extension: &XyzExtension,
    xyz_nft_id: &String,
    random_numbers: Vec<u8>,
    boost: u8,
    config: &Config,
    store: &dyn Storage,
    block: &BlockInfo,
) -> StdResult<Option<Planet>> {

    // If any boost is applied, we will automatically
    // grant a planet and apply the boost to remaining probabilities
    if boost == 0 && random_numbers[0] > config.probability_of_discovery {
        // No planet was discovered
        return Ok(None);
    }

    // Tracks all the resources granted to this planet
    let mut discovered_resources: Vec<PlanetResource> = Vec::new();

    // A generated planet should have at least one core resource.
    // We pick a core resource at random and remove it from the list.
    let core_resource_count = config.core_resource_generation_info.len().to_u8();
    let selected_core_resource_index = if core_resource_count.is_none() {
        0
    } else {
        random_numbers[1] % core_resource_count.unwrap()
    }
    .to_usize()
    .unwrap();

    let selected_core_resource = generate_resource_for_planet(
        0,                 // will guarantee that this resource will be granted.
        random_numbers[2], // richness will still be random.
        &config.core_resource_generation_info[selected_core_resource_index],
    );

    // Sanity check, this should always be true since we override the appearance probability
    if selected_core_resource.is_some() {
        discovered_resources.push(selected_core_resource.unwrap());
    }

    // Generate remaining resources based on the random numbers.
    let mut all_resource_generation_info = config.core_resource_generation_info.clone();
    all_resource_generation_info.remove(selected_core_resource_index);
    all_resource_generation_info.append(config.resource_generation_info.clone().as_mut());

    for (i, resource_gen_config) in all_resource_generation_info.iter().enumerate() {
        let generated_resource = generate_resource_for_planet(
            random_numbers[2 * i + 3].checked_sub(boost).unwrap_or(0), // Lower number is more desirable
            random_numbers[2 * i + 4].checked_add(boost).unwrap_or(u8::MAX), // Higher number is more desirable
            resource_gen_config,
        );
        // add the generated resource to be added to the planet
        if generated_resource.is_some() {
            let _ = &discovered_resources.push(generated_resource.unwrap());
        }
    }

    let current_contract_version = match get_contract_version(store) {
        Ok(version) => Some(version),
        Err(_) => None,
    };

    return Ok(Some(Planet {
        discovered_by: xyz_nft_id.to_string(),
        planet_id: None, // This will be set when the planet is saved.
        resources: discovered_resources,
        discovered_contract_version: current_contract_version,
        discovery_time: block.time,
        coordinates: PlanetCoordinates::from_xyz_coordinates(xyz_nft_extension.coordinates)?,
    }));
}

fn attempt_claim(
    task: &Task,
    xyz_nft_extension: &XyzExtension,
    boost: u8,
    querier: &QuerierWrapper,
    storage: &mut dyn Storage,
    block: &BlockInfo,
    config: &Config,
) -> StdResult<Option<Planet>> {
    // Fail if task is not complete yet
    if !task.is_task_complete(block) {
        return Err(StdError::generic_err("Task is still in progress"));
    }

    // Return Empty if task is expired. We will delete the task and not generate a planet.
    if task.is_task_expired(block) {
        return Ok(None);
    }

    let random_numbers: Vec<u8> = fetch_random_numbers(
        querier,
        &config.randomness_contract_address,
        config.required_seconds,
        &task.start_time,
        &task.nft_token_id
    )?;

    let planet_option = generate_planet(
        xyz_nft_extension,
        &task.nft_token_id,
        random_numbers,
        boost,
        config,
        storage,
        block,
    )?;

    if planet_option.is_some() {
        // Planet discovered for nft id
        let res: Planet = save_planet(&planet_option.unwrap(), storage, &block)?;
        return Ok(Some(res));
    } else {
        // Planet was not descovered due to probability
        return Ok(None);
    }
}

pub fn try_claim(
    xyz_nft_id: String,
    claimed_owner_addr: String,
    querier: &QuerierWrapper,
    storage: &mut dyn Storage,
    block: BlockInfo,
) -> Result<Response, StdError> {
    let config = CONFIG.load(storage)?;

    if !validate_nft_is_owned_by_wallet(
        &xyz_nft_id,
        &claimed_owner_addr,
        querier,
        &config.xyz_nft_contract_address,
    )? {
        return Err(StdError::generic_err("Wallet does not own NFT"));
    }
    // get the current task. Fail if there is no task in progress.
    let existing_task: Task = TASK_REPOSITORY.fetch_existing_task(&xyz_nft_id, storage)?;

    // Coordinate checks
    let nft_info = fetch_nft_data(
        &existing_task.nft_token_id,
        &config.xyz_nft_contract_address,
        querier,
    )?;

    // Cant claim if xyz is moving
    if !nft_info.extension.has_arrived(block.time) {
        return Err(StdError::generic_err(
            "Cannot complete a task for moving nft.",
        ));
    }

    // Claim nothing if NFT position does not match Task
    if PlanetCoordinates::from_xyz_coordinates(nft_info.extension.coordinates)? != existing_task.coordinates {
        TASK_REPOSITORY.remove_task(storage, &existing_task)?;
        return Ok(Response::default()
            .add_attribute("action", "no-op")
            .add_attribute("reason", "task coordinates dont match xyz coordinates"));
    }

    // Fail if the nft cannot discover more planets
    if is_planet_limit_reached(storage, &config, &existing_task.coordinates) {
        TASK_REPOSITORY.remove_task(storage, &existing_task)?;
        return Ok(
            Response::default()
                .add_attribute("action", "no-op")
                .add_attribute("reason", "planet limit has been reached")
        )
    }

    // Execute the claim + attempt to generate a planet & resources
    let planet =attempt_claim(
        &existing_task,
        &nft_info.extension,
        existing_task.expected_boost,
        querier,
        storage,
        &block,
        &config,
    )?;

    let mut planet_attributes: Vec<(String, String)>= vec![];
    if let Some(plan) = planet {
        planet_attributes.push(("planet".to_string(), serde_json::to_string(&plan).unwrap()));
    }
    
    // Delete the task once we have successfully processed it
    TASK_REPOSITORY.remove_task(storage, &existing_task)?;

    let mint_exp_info = config.experience_mint_config;
    let mint_exp_msg = mint_exp_info.mint_experince(
        xyz_nft_id.to_string(),
    )?;

    return Ok(
        Response::default()
            .add_attribute("method", "complete task")
            .add_attribute("xyz_id", xyz_nft_id.to_string())
            .add_attribute("task_type", "planet discovery")
            .add_attribute("experience_gained", mint_exp_info.complete_task_experience_amount)
            .add_attributes(planet_attributes)
            .add_message(mint_exp_msg)
    );
}
