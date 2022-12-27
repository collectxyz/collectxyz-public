use cosmwasm_std::{BlockInfo, MessageInfo, QuerierWrapper, Response, StdError, Storage};

use collectxyz_planet_metaverse::{discover_planets::{Planet, PlanetCoordinates, PlanetResource}, util::{fetch_nft_data, fetch_planets_by_coord, validate_nft_is_owned_by_wallet}};
use collectxyz_planet_metaverse::tasks::{Task};
use collectxyz_planet_metaverse::util::{check_sufficient_funds};

use crate::state::{CONFIG, NFT_ID_GATHERING_RESOURCES, TASK_REPOSITORY};

pub fn start_task(
    xyz_nft_id: &String,
    info: MessageInfo,
    block: &BlockInfo,
    storage: &mut dyn Storage,
    querier: &QuerierWrapper,
) -> Result<Response, StdError> {
    let config = CONFIG.load(storage)?;

    let claimed_xyz_owner_addr = info.sender.to_string();
    
    // check that task fee is covered
    check_sufficient_funds(info.funds, &config.start_task_fee)?;
    
    // Fail if the NFT is not owned by sender
    if !validate_nft_is_owned_by_wallet(
        xyz_nft_id,
        &claimed_xyz_owner_addr,
        querier,
        &config.xyz_nft_contract_address,
    )? {
        return Err(StdError::generic_err("Wallet does not own NFT"));
    }

    // Fail if a task already exists
    let existing_task_result = TASK_REPOSITORY.fetch_existing_task(xyz_nft_id, storage);
    if existing_task_result.is_ok() {
        return Err(StdError::generic_err("Gathering Task alrady in progress."));
    }

    /****** Coordinate checks ******/
    let nft_info = fetch_nft_data(
        &xyz_nft_id,
        &config.xyz_nft_contract_address,
        querier,
    )?;

    // Fail if xyz is moving
    if !nft_info.extension.has_arrived(block.time) {
        return Err(StdError::generic_err(
            "Cannot start a task for moving nft.",
        ));
    }

    let coordinates =
         &PlanetCoordinates::from_xyz_coordinates(nft_info.extension.coordinates)?;
    // Create the task
    let task = Task::new(
        &xyz_nft_id,
        &block.time,
        // No boost supported for resource gathering
        0,
        coordinates,
        config.gather_task_duration_seconds,
        config.gather_task_expiration_seconds
    );
    TASK_REPOSITORY.save_task(storage, &task)?;

    // Save list of resources that currently exist and are being gathered
    let planets: Vec<Planet> = fetch_planets_by_coord(
        querier,
        &config.planet_contract_address.to_string(),
        coordinates,
    )?;

    // Fail if the NFT's coordinates have no planets
    if planets.len() == 0 {
        return Err(StdError::generic_err("No Planets to begin gathering."))
    }

    let gathering_resources: Vec<PlanetResource> = planets
        .iter()
        .flat_map(|planet| planet.resources.clone())
        .collect();
    NFT_ID_GATHERING_RESOURCES.save(storage, xyz_nft_id, &gathering_resources)?;

    return Ok(
        Response::default()
            .add_attribute("method", "start task")
            .add_attribute("xyz_id", xyz_nft_id.to_string())
            .add_attribute("task_type", "gather resources")
    );
}