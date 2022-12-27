#![cfg(test)]

use std::collections::HashMap;

use collectxyz::nft::{XyzExtension, XyzTokenInfo};
use collectxyz_experience::CollectXyzExperienceExecuteMsg;
use collectxyz_planet_metaverse::discover_planets::{Planet, PlanetCoordinates, PlanetResource};
use collectxyz_planet_metaverse::experience::{XyzExperienceMintInfo};
use collectxyz_planet_metaverse::mock_querier::{DEFAULT_RAND, EXPERIENCE_CONTRACT_ADDRESS, NFT_CONTRACT_ADDRESS, NFT_OWNER_ADDRESS, NOW, PLANET_CONTRACT_ADDRESS, RANDOM_CONTRACT_ADDRESS, mock_dependencies_custom};
use collectxyz_planet_metaverse::tasks::Task;
use collectxyz_resources::CollectXyzResourceExecuteMsg;
use cosmwasm_std::{Addr, Coin, CosmosMsg, DepsMut, Env, StdError, Timestamp, Uint128, WasmMsg, from_binary};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cw2::ContractVersion;
use fixed::types::I20F12;
use rust_decimal::prelude::ToPrimitive;

use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::contract::{instantiate, query_config, execute};
use crate::state::{Config, NFT_ID_GATHERING_RESOURCES, ResourceGatherInfo, TASK_REPOSITORY, load_resource_gather_info, load_all_resource_gathering_info};

// Mock Owners
pub const NFT_NONE_OWNER_ADDRESS: &str = "nft_non_owner_address";
pub const XYZ_NFT_ID_1: &str = "xyz 1";
pub const XYZ_NFT_ID_2: &str = "xyz 2";

// Default Config Data
pub const GATHER_RESOURCE_TASK_DURATION_SECONDS: u64 = 120;
pub const GATHER_RESOURCE_TASK_EXPIRATION_SECONDS: u64 = 240;

// Default resource gather data
pub const XYZ_ROCK_CONTRACT_ADDRESS: &str = "trxyzRockContractAddress";
pub const XYZ_METAL_CONTRACT_ADDRESS: &str = "trxyzMetalContractAddress";
pub const XYZ_ICE_CONTRACT_ADDRESS: &str = "trxyzIceContractAddress";

pub const XYZ_ROCK: &str = "xyzROCK";
pub const XYZ_ICE: &str = "xyzICE";
pub const XYZ_METAL: &str = "xyzMETAL";

pub const BASE_YIELD: u64 = 10;
pub const MAX_DEVIATION_YIELD: u64 = 5;
pub const DEVIATION_DIR_THRESHOLD: u8 = 90;
pub const BONUS_TOKEN_PROBABILITY: u8 = 16;

// Temporal
pub const TWO_DAYS: u64 = 60*60*24*2;

fn setup_contract(deps: DepsMut) -> InstantiateMsg {
    let config = Config {
        planet_contract_address: Addr::unchecked(PLANET_CONTRACT_ADDRESS.to_string()),
        randomness_contract_address: Addr::unchecked(RANDOM_CONTRACT_ADDRESS.to_string()),
        xyz_nft_contract_address: Addr::unchecked(NFT_CONTRACT_ADDRESS.to_string()),
        gather_task_duration_seconds: GATHER_RESOURCE_TASK_DURATION_SECONDS,
        gather_task_expiration_seconds: GATHER_RESOURCE_TASK_EXPIRATION_SECONDS,
        bonus_token_probability: BONUS_TOKEN_PROBABILITY,
        start_task_fee: Coin::new(100, "uluna"),
        experience_mint_config: XyzExperienceMintInfo { 
            experience_contract_address: Addr::unchecked(EXPERIENCE_CONTRACT_ADDRESS),
            complete_task_experience_amount: Uint128::from(1u128),
        }
    };

    let resource_gathering_info = vec![
        ResourceGatherInfo {
            resource_identifier: XYZ_ROCK.to_string(),
            resource_contract_address: XYZ_ROCK_CONTRACT_ADDRESS.to_string(),
            base_yield: BASE_YIELD, 
            max_deviation_yield: MAX_DEVIATION_YIELD,
            deviation_direction_threshhold: DEVIATION_DIR_THRESHOLD,
        },
        ResourceGatherInfo {
            resource_identifier: XYZ_ICE.to_string(),
            resource_contract_address: XYZ_ICE_CONTRACT_ADDRESS.to_string(),
            base_yield: BASE_YIELD, 
            max_deviation_yield: MAX_DEVIATION_YIELD,
            deviation_direction_threshhold: DEVIATION_DIR_THRESHOLD,
        },
        ResourceGatherInfo {
            resource_identifier: XYZ_METAL.to_string(),
            resource_contract_address: XYZ_METAL_CONTRACT_ADDRESS.to_string(),
            base_yield: BASE_YIELD, 
            max_deviation_yield: MAX_DEVIATION_YIELD,
            deviation_direction_threshhold: DEVIATION_DIR_THRESHOLD,
        },
    ];

    let msg = InstantiateMsg {
        config,
        resource_gathering_info
    };
    let _ = instantiate(
        deps,
        mock_env(),
        mock_info(NFT_OWNER_ADDRESS, &[]),
        msg.clone(),
    )
    .unwrap();

    msg
}

fn mock_env_block_time(ts: Timestamp) -> Env {
    let mut env = mock_env();
    env.block.time = ts;
    env
}

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies(&[]);
    let msg = setup_contract(deps.as_mut());

    // Query to make sure the contract's config was updated
    let result = query_config(deps.as_mut().storage).unwrap();
    assert_eq!(
        result,
        msg.config
    );

    let resource_gather_info = load_all_resource_gathering_info(deps.as_mut().storage).unwrap();

    // Test mappings are created for resources
    for resource in msg.resource_gathering_info {
        let stored_resource_gather_info = resource_gather_info.get(&resource.resource_identifier);
        assert_eq!(
            stored_resource_gather_info.is_some(),
            true
        );
        assert_eq!(
            resource,
            *stored_resource_gather_info.unwrap()
        );
    }
}

// Test Start Gather
#[test]
fn test_start_task() {
    let owner = NFT_OWNER_ADDRESS.to_string();
    let non_owner = NFT_NONE_OWNER_ADDRESS.to_string();
    let xyz_nft_id = &XYZ_NFT_ID_1.to_string();
    let now_timestamp = Timestamp::from_seconds(NOW);

    let coordinates = PlanetCoordinates {
        x: 100,
        y: 100,
        z: 100
    };

    let arrived_nft_info =
        default_xyz_nft_data(now_timestamp.seconds(), true, Some(coordinates));
    let moving_nft_info =
        default_xyz_nft_data(now_timestamp.seconds(), false, Some(coordinates));

    let rock_resource = PlanetResource {
        resource_identifier: XYZ_ROCK.to_string(),
        resource_richness_score: 1,
    };

    let iron_resource = PlanetResource {
        resource_identifier: XYZ_METAL.to_string(),
        resource_richness_score: 1,
    };

    let ice_resource = PlanetResource {
        resource_identifier: XYZ_ICE.to_string(),
        resource_richness_score: 1,
    };

    let planet_id = "Planet 1".to_string();
    let planet = Planet {
        discovered_by: String::from(xyz_nft_id),
        planet_id: Some(planet_id),
        resources: vec![rock_resource.clone(), ice_resource.clone(), iron_resource.clone()],
        discovery_time: Timestamp::from_seconds(1633193676),
        discovered_contract_version: Some(default_contract_version()),
        coordinates: coordinates
    };

    let _env = mock_env_block_time(now_timestamp);
    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(arrived_nft_info.clone()),
        vec![],
        &[],
    );

    let init_msg = setup_contract(deps.as_mut());

    /********************* FAILURE *********************/
    // Fail if not owner
    let mut deps =
        mock_dependencies_custom(Some(non_owner.to_string()), Some(DEFAULT_RAND), None, vec![], &[]);

    let _ = setup_contract(deps.as_mut());
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[Coin::new(100, "uluna")]),
        start_task_msg,
    );
    assert_eq!(execute_result.is_err(), true);
    assert_eq!(
        execute_result.unwrap_err(),
        StdError::generic_err("Wallet does not own NFT")
    );

    // Fail if XYZ is moving
    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(moving_nft_info),
        vec![],
        &[],
    );

    let _ = setup_contract(deps.as_mut());
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[Coin::new(100, "uluna")]),
        start_task_msg,
    );
    assert_eq!(execute_result.is_err(), true);
    assert_eq!(
        execute_result.unwrap_err(),
        StdError::generic_err("Cannot start a task for moving nft.")
    );

    // Fail if there are no planets for coord
    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(arrived_nft_info.clone()),
        vec![],
        &[],
    );

    let _ = setup_contract(deps.as_mut());
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[Coin::new(100, "uluna")]),
        start_task_msg,
    );
    assert_eq!(execute_result.is_err(), true);
    assert_eq!(
        execute_result.unwrap_err(),
        StdError::generic_err("No Planets to begin gathering.")
    );

    /* Start task as owner with insufficient funds */
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[]),
        start_task_msg,
    );
    assert_eq!(execute_result.is_err(), true);
    assert_eq!(
        execute_result.unwrap_err(),
        StdError::generic_err("insufficient funds sent")
    );
    // Fail if there is already a task
    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(arrived_nft_info.clone()),
        vec![planet.clone()],
        &[],
    );

    let _ = setup_contract(deps.as_mut());
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let _ = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[Coin::new(100, "uluna")]),
        start_task_msg.clone(),
    );

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[Coin::new(100, "uluna")]),
        start_task_msg,
    );
    assert_eq!(execute_result.is_err(), true);
    assert_eq!(
        execute_result.unwrap_err(),
        StdError::generic_err("Gathering Task alrady in progress.")
    );

    /********************* SUCCESS *********************/
    // Successful start gathering task
    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(arrived_nft_info.clone()),
        vec![planet.clone()],
        &[],
    );

    let _ = setup_contract(deps.as_mut());
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[Coin::new(100, "uluna")]),
        start_task_msg,
    );
    assert_eq!(execute_result.is_ok(), true);

    let task = TASK_REPOSITORY.fetch_existing_task(&xyz_nft_id, deps.as_mut().storage).unwrap();
    assert_eq!(
        task.completes,
        now_timestamp.plus_seconds(init_msg.config.gather_task_duration_seconds)
    );
    assert_eq!(
        task.expires,
        now_timestamp.plus_seconds(init_msg.config.gather_task_expiration_seconds)
    );
    assert_eq!(
        task.nft_token_id,
        xyz_nft_id.to_string()
    );
    assert_eq!(
        task.coordinates,
        planet.coordinates
    );
     
    let resources = NFT_ID_GATHERING_RESOURCES.load(deps.as_mut().storage, &xyz_nft_id).unwrap();
    for resource in planet.resources {
        assert_eq!(
            resources.contains(&resource),
            true
        );
    }
}

// Test Complete Gather
#[test]
fn test_complete_task() {
    let owner = NFT_OWNER_ADDRESS.to_string();
    let non_owner = NFT_NONE_OWNER_ADDRESS.to_string();
    let xyz_nft_id = &XYZ_NFT_ID_1.to_string();
    let now_timestamp = Timestamp::from_seconds(NOW);

    let coordinates = PlanetCoordinates {
        x: 100,
        y: 100,
        z: 100
    };

    let moved_coordinates = PlanetCoordinates {
        x: 200,
        y: 200,
        z: 200
    };

    let arrived_nft_info =
        default_xyz_nft_data(now_timestamp.seconds(), true, Some(coordinates));
    let moving_nft_info =
        default_xyz_nft_data(now_timestamp.seconds(), false, Some(coordinates));
    let moved_nft_info =
        default_xyz_nft_data(now_timestamp.seconds(), true, Some(moved_coordinates));

    let rock_resource = PlanetResource {
        resource_identifier: XYZ_ROCK.to_string(),
        resource_richness_score: 1,
    };

    let metal_resource = PlanetResource {
        resource_identifier: XYZ_METAL.to_string(),
        resource_richness_score: 1,
    };

    let ice_resource = PlanetResource {
        resource_identifier: XYZ_ICE.to_string(),
        resource_richness_score: 1,
    };

    let complete_task = Task {
        nft_token_id: xyz_nft_id.to_string(),
        start_time: now_timestamp.minus_seconds(TWO_DAYS),
        completes: now_timestamp,
        expires: now_timestamp.plus_seconds(TWO_DAYS),
        expected_boost: 0,
        coordinates: coordinates,
    };

    let expired_task = Task {
        nft_token_id: xyz_nft_id.to_string(),
        start_time: now_timestamp.minus_seconds(TWO_DAYS*2),
        completes: now_timestamp.minus_seconds(TWO_DAYS),
        expires: now_timestamp,
        expected_boost: 0,
        coordinates: coordinates,
    };

    let incomplete_task = Task {
        nft_token_id: xyz_nft_id.to_string(),
        start_time: now_timestamp,
        completes: now_timestamp.plus_seconds(TWO_DAYS),
        expires: now_timestamp.plus_seconds(TWO_DAYS*2),
        expected_boost: 0,
        coordinates: coordinates,
    };

    let _env = mock_env_block_time(now_timestamp);
    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(arrived_nft_info.clone()),
        vec![],
        &[],
    );

    let _init_msg = setup_contract(deps.as_mut());

    /********************* FAILURE *********************/
    // Fail if not owner
    let mut deps =
        mock_dependencies_custom(Some(non_owner.to_string()), Some(DEFAULT_RAND), None, vec![], &[]);

    let _ = setup_contract(deps.as_mut());
    let complete_task_msg = ExecuteMsg::CompleteTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[]),
        complete_task_msg,
    );
    assert_eq!(execute_result.is_err(), true);
    assert_eq!(
        execute_result.unwrap_err(),
        StdError::generic_err("Wallet does not own NFT")
    );

    // Fail if no task exists
    let mut deps =
        mock_dependencies_custom(Some(owner.to_string()), Some(DEFAULT_RAND), None, vec![], &[]);

    let _ = setup_contract(deps.as_mut());
    let complete_task_msg = ExecuteMsg::CompleteTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[]),
        complete_task_msg,
    );
    assert_eq!(execute_result.is_err(), true);
    assert_eq!(
        execute_result.unwrap_err(),
        StdError::generic_err("No task in progress.")
    );

    // Fail if not complete
    let mut deps =
        mock_dependencies_custom(Some(owner.to_string()), Some(DEFAULT_RAND), None, vec![], &[]);

    let _ = setup_contract(deps.as_mut());
    let complete_task_msg = ExecuteMsg::CompleteTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &incomplete_task);

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[]),
        complete_task_msg,
    );
    assert_eq!(execute_result.is_err(), true);
    assert_eq!(
        execute_result.unwrap_err(),
        StdError::generic_err("Task is not completed yet")
    );

    // Fail if moving
    let mut deps =
        mock_dependencies_custom(Some(owner.to_string()), Some(DEFAULT_RAND), Some(moving_nft_info), vec![], &[]);

    let _ = setup_contract(deps.as_mut());
    let complete_task_msg = ExecuteMsg::CompleteTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &complete_task);

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[]),
        complete_task_msg,
    );
    assert_eq!(execute_result.is_err(), true);
    assert_eq!(
        execute_result.unwrap_err(),
        StdError::generic_err("Cannot complete a task for moving nft.")
    );

    // No-op if current coords dont match task
    let mut deps =
        mock_dependencies_custom(Some(owner.to_string()), Some(DEFAULT_RAND), Some(moved_nft_info), vec![], &[]);

    let _ = setup_contract(deps.as_mut());
    let complete_task_msg = ExecuteMsg::CompleteTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &complete_task);

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[]),
        complete_task_msg,
    );
    assert_eq!(execute_result.is_ok(), true);
    
    let execute_result = execute_result.unwrap();
    let task_result = TASK_REPOSITORY.fetch_existing_task(xyz_nft_id, deps.as_mut().storage);
    assert_eq!(
        task_result.is_err(),
        true
    );
    assert_eq!(
        execute_result.messages.len(),
        0
    );

    // Fail if no resources found for nft id
    let mut deps =
        mock_dependencies_custom(Some(owner.to_string()), Some(DEFAULT_RAND), Some(arrived_nft_info.clone()), vec![], &[]);

    let _ = setup_contract(deps.as_mut());
    let complete_task_msg = ExecuteMsg::CompleteTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &complete_task);

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[]),
        complete_task_msg,
    );
    assert_eq!(
        execute_result.is_err(),
        true
    );
    assert_eq!(
        execute_result.unwrap_err(),
        StdError::NotFound { kind: "alloc::vec::Vec<collectxyz_planet_metaverse::discover_planets::PlanetResource>".to_string() }
    );

    /********************* SUCCESS *********************/
    let mut expected_resource_contracts_to_id: HashMap<String, String> = HashMap::new();
    expected_resource_contracts_to_id.insert(XYZ_ROCK_CONTRACT_ADDRESS.to_string(), rock_resource.resource_identifier.clone());
    expected_resource_contracts_to_id.insert(XYZ_METAL_CONTRACT_ADDRESS.to_string(), metal_resource.resource_identifier.clone());
    expected_resource_contracts_to_id.insert(XYZ_ICE_CONTRACT_ADDRESS.to_string(), ice_resource.resource_identifier.clone());
    // Return min yield if expired
    let mut deps =
        mock_dependencies_custom(Some(owner.to_string()), Some(DEFAULT_RAND), Some(arrived_nft_info.clone()), vec![], &[]);

    let _ = setup_contract(deps.as_mut());
    let complete_task_msg = ExecuteMsg::CompleteTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &expired_task);
    let _ = NFT_ID_GATHERING_RESOURCES.save(
        deps.as_mut().storage, 
        &xyz_nft_id, 
        &vec![rock_resource.clone(), metal_resource.clone(), ice_resource.clone()]
    );

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[]),
        complete_task_msg,
    );
    assert_eq!(execute_result.is_ok(), true);
    
    let mut execute_result = execute_result.unwrap();
    let task_result = TASK_REPOSITORY.fetch_existing_task(xyz_nft_id, deps.as_mut().storage);
    assert_eq!(
        task_result.is_err(),
        true
    );
    assert_eq!(
        execute_result.messages.len(),
        4
    );

    let experience_message = execute_result.messages.pop().unwrap();
    match experience_message.msg {
        CosmosMsg::Wasm(WasmMsg::Execute {contract_addr: _, msg, ..}, ..) => {
            match from_binary(&msg).unwrap() {
                CollectXyzExperienceExecuteMsg::Mint {recipient_xyz_id, amount} => {
                    assert_eq!(
                        recipient_xyz_id.eq(xyz_nft_id),
                        true
                    );
                    assert_eq!(
                        Uint128::from(1u128*1000000),
                        amount
                    )
                },
                _ => {
                    panic!("DO NOT ENTER HERE")
                }
            }
        },
        _ => { panic!("DO NOT ENTER HERE") }
    }

    for message in execute_result.messages {
        match &message.msg {
            CosmosMsg::Wasm(WasmMsg::Execute {contract_addr, msg, ..}, ..)=> {
                match from_binary(&msg).unwrap() {
                    CollectXyzResourceExecuteMsg::Mint {recipient_xyz_id, amount} => {
                        let resource_id = expected_resource_contracts_to_id.get(contract_addr).unwrap();
                        let resource_gather_info = load_resource_gather_info(deps.as_mut().storage, resource_id).unwrap();
                        assert_eq!(
                            recipient_xyz_id.eq(xyz_nft_id),
                            true
                        );
                        assert_eq!(
                            expected_resource_contracts_to_id.contains_key(contract_addr),
                            true
                        );
                        // This is minimum yiel BASE_YIELD = 10, MAX_DEVIATION = 10% so min_yield = 10 - (10*0.1) = 9
                        assert_eq!(
                            Uint128::from((resource_gather_info.base_yield - resource_gather_info.max_deviation_yield)*1000000),
                            amount
                        )
                    },
                    _ => {
                        panic!("DO NOT ENTER HERE")
                    }
                }
            },
            _ => {}
        }
    }

    let mut resource_yield_map: HashMap<String, Uint128> = HashMap::new();
    resource_yield_map.insert("xyzMETAL".to_string(), Uint128::from((BASE_YIELD - MAX_DEVIATION_YIELD) * 1000000));
    resource_yield_map.insert("xyzICE".to_string(), Uint128::from((BASE_YIELD - MAX_DEVIATION_YIELD) * 1000000));
    resource_yield_map.insert("xyzROCK".to_string(), Uint128::from((BASE_YIELD - MAX_DEVIATION_YIELD) * 1000000));
    for attribute in execute_result.attributes {
        match attribute.key.as_str() {
            "resources_gathered" => {
                println!("{:?}", attribute);
                let map = serde_json::from_str::<HashMap<String, Uint128>>(&attribute.value);
                assert_eq!(map.unwrap(), resource_yield_map);
            }
            _ => {
                println!("{:?}", attribute);
            }
        }
    }

    // Apply extra yield from random numbers
    /*
        first three random numbers used after txn hash: [122, 108, 121]
    */
    let mut deps =
        mock_dependencies_custom(Some(owner.to_string()), Some(&[10, 101]), Some(arrived_nft_info.clone()), vec![], &[]);

    let _ = setup_contract(deps.as_mut());
    let complete_task_msg = ExecuteMsg::CompleteTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &complete_task);
    let _ = NFT_ID_GATHERING_RESOURCES.save(
        deps.as_mut().storage, 
        &xyz_nft_id, 
        &vec![rock_resource.clone()]
    );

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[]),
        complete_task_msg,
    );
    assert_eq!(execute_result.is_ok(), true);
    
    let mut execute_result = execute_result.unwrap();
    let task_result = TASK_REPOSITORY.fetch_existing_task(xyz_nft_id, deps.as_mut().storage);
    assert_eq!(
        task_result.is_err(),
        true
    );
    assert_eq!(
        execute_result.messages.len(),
        2
    );

    // remove last message which is the experience mint
    let experience_message = execute_result.messages.pop().unwrap();
    match experience_message.msg {
        CosmosMsg::Wasm(WasmMsg::Execute {contract_addr: _, msg, ..}, ..) => {
            match from_binary(&msg).unwrap() {
                CollectXyzExperienceExecuteMsg::Mint {recipient_xyz_id, amount} => {
                    assert_eq!(
                        recipient_xyz_id.eq(xyz_nft_id),
                        true
                    );
                    assert_eq!(
                        Uint128::from(1u128*1000000),
                        amount
                    )
                },
                _ => {
                    panic!("DO NOT ENTER HERE")
                }
            }
        },
        _ => { panic!("DO NOT ENTER HERE") }
    }
    for message in execute_result.messages {
        match &message.msg {
            CosmosMsg::Wasm(WasmMsg::Execute {contract_addr, msg, ..}, ..)=> {
                match from_binary(&msg).unwrap() {
                    CollectXyzResourceExecuteMsg::Mint {recipient_xyz_id, amount} => {
                        let resource_id = expected_resource_contracts_to_id.get(contract_addr).unwrap();
                        let resource_gather_info = load_resource_gather_info(deps.as_mut().storage, resource_id).unwrap();
                        assert_eq!(
                            recipient_xyz_id.eq(xyz_nft_id),
                            true
                        );
                        assert_eq!(
                            expected_resource_contracts_to_id.contains_key(contract_addr),
                            true
                        );
                        let deviation = I20F12::from_num(resource_gather_info.max_deviation_yield) * 1.to_i32().unwrap() * 122.to_i32().unwrap() / u8::MAX.to_i32().unwrap();
                        assert_eq!(
                            Uint128::from((resource_gather_info.base_yield + deviation.floor().to_num::<u64>())*1000000),
                            amount
                        )
                    },
                    _ => {
                        panic!("DO NOT ENTER HERE")
                    }
                }
            },
            _ => {}
        }
    }
}

// Helper Functions
pub fn default_xyz_nft_data(now_seconds: u64, has_arrived: bool, coordinates: Option<PlanetCoordinates>) -> XyzTokenInfo {
    return XyzTokenInfo {
        owner: Addr::unchecked(NFT_OWNER_ADDRESS.to_string()),
        approvals: vec![],
        description: "".to_string(),
        image: None,
        name: "".to_string(),
        extension: default_xyz_extension(now_seconds, has_arrived, coordinates)
    }
}

pub fn default_xyz_extension(now_seconds: u64, has_arrived: bool, coordinates: Option<PlanetCoordinates>) -> XyzExtension {
    return XyzExtension {
        coordinates: PlanetCoordinates::to_xyz_coordinates(&coordinates.unwrap_or(default_xyz_coords())).unwrap(),
        prev_coordinates: None,
        arrival: Timestamp::from_seconds(if has_arrived { now_seconds - 1 } else { now_seconds + 1 })
    }
}

pub fn default_contract_version() -> ContractVersion {
    return ContractVersion {
        contract: String::from("contract"),
        version: String::from("v1.1"),
    };
}

pub fn default_xyz_coords() -> PlanetCoordinates {
    return PlanetCoordinates {
        x: 100,
        y: 100,
        z: 100
    };
}