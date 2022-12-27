#![cfg(test)]

use collectxyz_planet_metaverse::discover_planets::{PlanetCoordinates, Planet};
use collectxyz_planet_metaverse::experience::XyzExperienceMintInfo;
use collectxyz_planet_metaverse::mock_querier::{DEFAULT_RAND, EXPERIENCE_CONTRACT_ADDRESS, NFT_CONTRACT_ADDRESS, NFT_OWNER_ADDRESS, NOW, RANDOM_CONTRACT_ADDRESS, default_xyz_coords, default_xyz_nft_data, mock_dependencies_custom};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{Addr, Coin, DepsMut, Env, StdError, Timestamp, Uint128};
use rust_decimal::prelude::ToPrimitive;

use crate::planet_repository::{fetch_all_planets_for_coordinate, save_planet};
use crate::planet_util::query_all_planets_for_coord;
use crate::state::{TASK_REPOSITORY};
use collectxyz_planet_metaverse::tasks::{Task};
use crate::start_task::query_task_for_nft;
use crate::test_helpers::{DEFAULT_BOOST_PER_BONUS_TOKEN, DEFAULT_CW20_BONUS_TOKEN_CONTRACT, DEFAULT_DISCOVERY_EXPIRATION_WINDOW, DEFAULT_MAX_BONUS_TOKEN_COUNT, MAX_ALLOWED_PLANETS, NFT_OWNER_ADDRESS_2, TWO_DAYS, XYZ_NFT_ID, default_planet_by_coord, default_resource_generation_info_with_id, default_xyz_coords_2};

use crate::contract::{execute, instantiate, query_config};
use crate::msg::{ExecuteMsg, InstantiateMsg};

fn setup_contract(deps: DepsMut) -> InstantiateMsg {
    let msg = InstantiateMsg {
        probability_of_discovery: 200,
        required_seconds: TWO_DAYS,
        resource_generation_info: vec![
            default_resource_generation_info_with_id("xyzLIFE"),
            default_resource_generation_info_with_id("xyzGEM"),
        ],
        core_resource_generation_info: vec![
            default_resource_generation_info_with_id("xyzMETAL"),
            default_resource_generation_info_with_id("xyzROCK"),
        ],
        maximum_planets_per_coord: MAX_ALLOWED_PLANETS,
        randomness_contract_address: Addr::unchecked(RANDOM_CONTRACT_ADDRESS),
        xyz_nft_contract_address: Addr::unchecked(NFT_CONTRACT_ADDRESS),
        discovery_task_expiration_window_seconds: DEFAULT_DISCOVERY_EXPIRATION_WINDOW,
        max_number_of_bonus_tokens: DEFAULT_MAX_BONUS_TOKEN_COUNT,
        boost_per_bonus_token: DEFAULT_BOOST_PER_BONUS_TOKEN,
        cw20_bonus_token_contract: Addr::unchecked(DEFAULT_CW20_BONUS_TOKEN_CONTRACT),
        start_task_fee: Coin::new(100, "uluna"),
        experience_mint_config: XyzExperienceMintInfo {
            experience_contract_address: Addr::unchecked(EXPERIENCE_CONTRACT_ADDRESS.to_string()),
            complete_task_experience_amount: Uint128::from(0u128),
        }
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
        result.probability_of_discovery,
        msg.probability_of_discovery
    );
    assert_eq!(
        result.core_resource_generation_info,
        msg.core_resource_generation_info
    );
    assert_eq!(
        result.resource_generation_info,
        msg.resource_generation_info
    );
    assert_eq!(
        result.max_number_of_bonus_tokens,
        msg.max_number_of_bonus_tokens
    );
    assert_eq!(
        result.maximum_planets_per_coord,
        msg.maximum_planets_per_coord
    );
    assert_eq!(result.required_seconds, msg.required_seconds);
    assert_eq!(
        result.randomness_contract_address,
        msg.randomness_contract_address
    );
    assert_eq!(
        result.cw20_bonus_token_contract,
        msg.cw20_bonus_token_contract
    );
    assert_eq!(
        result.xyz_nft_contract_address,
        msg.xyz_nft_contract_address
    );
    assert_eq!(
        result.start_task_fee,
        msg.start_task_fee
    );
}

#[test]
fn test_start_task() {
    let owner = NFT_OWNER_ADDRESS.to_string();
    let non_owner = NFT_OWNER_ADDRESS_2.to_string();
    let xyz_nft_id = XYZ_NFT_ID.to_string();
    let now_timestamp = Timestamp::from_seconds(NOW);
    let arrived_nft_info =
        default_xyz_nft_data(now_timestamp.seconds(), true, Some(default_xyz_coords()));
    let moving_nft_info =
        default_xyz_nft_data(now_timestamp.seconds(), false, Some(default_xyz_coords()));

    let _env = mock_env_block_time(now_timestamp);
    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(arrived_nft_info.clone()),
        vec![],
        &[],
    );

    let init_msg = setup_contract(deps.as_mut());

    // ***************************** PASS CASES *****************************

    /* Start task as owner with 0 bonus tokens */
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: XYZ_NFT_ID.to_string(),
        bonus_token_count: 0,
    };

    let _ = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[Coin::new(100, "uluna")]),
        start_task_msg,
    )
    .unwrap();

    let task = query_task_for_nft(deps.as_mut().storage, &xyz_nft_id).unwrap();
    assert_eq!(task.is_some(), true);
    let _task = task.unwrap();
    assert_eq!(_task.coordinates, PlanetCoordinates::from_xyz_coordinates(moving_nft_info.extension.coordinates).unwrap());
    assert_eq!(
        _task.expires,
        now_timestamp.plus_seconds(init_msg.discovery_task_expiration_window_seconds)
    );
    assert_eq!(
        _task.completes,
        now_timestamp.plus_seconds(init_msg.required_seconds)
    );
    assert_eq!(_task.expected_boost, 0);
    let _ = TASK_REPOSITORY.remove_task(deps.as_mut().storage, &_task);

    /* Start task as owner with 1 bonus tokens */
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: XYZ_NFT_ID.to_string(),
        bonus_token_count: 1,
    };

    let _ = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[Coin::new(100, "uluna")]),
        start_task_msg,
    )
    .unwrap();

    let task = query_task_for_nft(deps.as_mut().storage, &xyz_nft_id).unwrap();
    assert_eq!(task.is_some(), true);
    let _task = task.unwrap();
    assert_eq!(_task.coordinates, PlanetCoordinates::from_xyz_coordinates(moving_nft_info.extension.coordinates).unwrap());
    assert_eq!(
        _task.expires,
        now_timestamp.plus_seconds(init_msg.discovery_task_expiration_window_seconds)
    );
    assert_eq!(
        _task.completes,
        now_timestamp.plus_seconds(init_msg.required_seconds)
    );
    assert_eq!(_task.expected_boost, 1 * init_msg.boost_per_bonus_token);
    let _ = TASK_REPOSITORY.remove_task(deps.as_mut().storage, &_task);

    // ***************************** FAIL CASES *****************************

    /* Start task as owner with insufficient funds */
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: XYZ_NFT_ID.to_string(),
        bonus_token_count: init_msg.max_number_of_bonus_tokens + 1,
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
    
    /* Start task as owner with > 1 bonus token */
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: XYZ_NFT_ID.to_string(),
        bonus_token_count: init_msg.max_number_of_bonus_tokens + 1,
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
        StdError::generic_err("More bonus tokens than allowed for a task boost.")
    );

    /* Start task as non-owner */
    let mut deps =
        mock_dependencies_custom(Some(non_owner.to_string()), Some(DEFAULT_RAND), None, vec![], &[]);

    let _ = setup_contract(deps.as_mut());
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: XYZ_NFT_ID.to_string(),
        bonus_token_count: 0,
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
    /* Start task for moving NFT */
    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(moving_nft_info),
        vec![],
        &[],
    );

    let _ = setup_contract(deps.as_mut());
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: XYZ_NFT_ID.to_string(),
        bonus_token_count: 0,
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

    /* Start task for coord with > maximum_planets_per_coord */
    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(arrived_nft_info),
        vec![],
        &[],
    );

    let init_msg = setup_contract(deps.as_mut());
    let start_task_msg = ExecuteMsg::StartTask {
        xyz_nft_id: XYZ_NFT_ID.to_string(),
        bonus_token_count: 0,
    };

    let coord = default_xyz_coords();
    default_planet_by_coord(&coord, Some("planet_1".to_string()));
    for i in 1..=init_msg.maximum_planets_per_coord {
        let _ = save_planet(
            &default_planet_by_coord(&coord, Some(i.to_string())),
            deps.as_mut().storage,
            &_env.block,
        );
    }

    let execute_result = execute(
        deps.as_mut(),
        mock_env_block_time(now_timestamp),
        mock_info(&owner, &[Coin::new(100, "uluna")]),
        start_task_msg,
    );
    assert_eq!(execute_result.is_err(), true);
    assert_eq!(
        execute_result.unwrap_err(),
        StdError::generic_err("Max allowed number of planets discovered!",)
    );
}

#[test]
fn test_comlete_task() {
    let owner = NFT_OWNER_ADDRESS.to_string();
    let non_owner = NFT_OWNER_ADDRESS_2.to_string();

    let xyz_nft_id = XYZ_NFT_ID.to_string();
    let xyz_coord = default_xyz_coords();
    let alternate_xyz_coord = default_xyz_coords_2();

    let now_timestamp = Timestamp::from_seconds(NOW);

    let completed_task = Task {
        nft_token_id: xyz_nft_id.to_string(),
        start_time: now_timestamp.minus_seconds(TWO_DAYS),
        expires: now_timestamp.plus_seconds(TWO_DAYS),
        completes: now_timestamp,
        expected_boost: 0,
        coordinates: xyz_coord.clone(),
    };

    let completed_task_expired = Task {
        nft_token_id: xyz_nft_id.to_string(),
        start_time: now_timestamp.minus_seconds(TWO_DAYS),
        expires: now_timestamp,
        completes: now_timestamp.minus_seconds(TWO_DAYS),
        expected_boost: 0,
        coordinates: xyz_coord.clone(),
    };

    let non_completed_task = Task {
        nft_token_id: xyz_nft_id.to_string(),
        start_time: now_timestamp.minus_seconds(TWO_DAYS),
        expires: now_timestamp.plus_seconds(TWO_DAYS),
        completes: now_timestamp.plus_seconds(TWO_DAYS),
        expected_boost: 0,
        coordinates: xyz_coord.clone(),
    };

    let arrived_nft_info =
        default_xyz_nft_data(now_timestamp.seconds(), true, Some(xyz_coord.clone()));
    let moving_nft_info =
        default_xyz_nft_data(now_timestamp.seconds(), false, Some(xyz_coord.clone()));
    let moved_nft_info = default_xyz_nft_data(
        now_timestamp.seconds(),
        true,
        Some(alternate_xyz_coord.clone()),
    );

    let _env = mock_env_block_time(now_timestamp);

    let msg = ExecuteMsg::CompleteTask {
        xyz_nft_id: xyz_nft_id.to_string(),
    };

    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(arrived_nft_info.clone()),
        vec![],
        &[],
    );

    let init_msg = setup_contract(deps.as_mut());

    let mut no_planet_rands = DEFAULT_RAND.clone().to_vec();
    no_planet_rands[0] = init_msg.probability_of_discovery + 1; //make sure we cant get planet without boost

    // ***************************** PASS CASES *****************************

    /* Claim task that is completed as owner with no boost */
    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &completed_task);
    let result = execute(
        deps.as_mut(),
        _env.clone(),
        mock_info(&owner, &[]),
        msg.clone(),
    );
    assert_eq!(result.is_ok(), true);
    
    let saved_planet =
        query_all_planets_for_coord(deps.as_mut().storage, &xyz_coord, None, None).unwrap();

    for attribute in result.unwrap().attributes {
        match attribute.key.as_str() {
            "planet" => {
                println!("{:?}", attribute);
                assert_eq!(saved_planet.claimed_planets[0], serde_json::from_str::<Planet>(&attribute.value).unwrap());
            }
            _ => {
                println!("{:?}", attribute);
            }
        }
    }
    
    assert_eq!(saved_planet.claimed_planets.len(), 0);

    /* Claim task that is completed as owner from a different coord: Return empty */
    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(moved_nft_info.clone()),
        vec![],
        &[],
    );

    let _ = setup_contract(deps.as_mut());
    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &completed_task);
    let result = execute(
        deps.as_mut(),
        _env.clone(),
        mock_info(&owner, &[]),
        msg.clone(),
    );

    assert_eq!(result.is_ok(), true);

    let saved_planet =
        query_all_planets_for_coord(deps.as_mut().storage, &xyz_coord, None, None).unwrap();

    assert_eq!(saved_planet.claimed_planets.len(), 0);
    /* Claim task that is complete as owner with maximum planet limit reached: Return empty */
    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(moved_nft_info.clone()),
        vec![],
        &[],
    );

    let init_msg = setup_contract(deps.as_mut());
    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &completed_task);
    // add planets for this coord to reach the max
    for i in 1..=init_msg.maximum_planets_per_coord {
        let _ = save_planet(
            &default_planet_by_coord(&xyz_coord, Some(i.to_string())),
            deps.as_mut().storage,
            &_env.block,
        );
    }

    let result = execute(
        deps.as_mut(),
        _env.clone(),
        mock_info(&owner, &[]),
        msg.clone(),
    );

    assert_eq!(result.is_ok(), true);

    let saved_planet =
        query_all_planets_for_coord(deps.as_mut().storage, &xyz_coord, None, None).unwrap();

    assert_eq!(
        saved_planet.claimed_planets.len().to_u8().unwrap(),
        init_msg.maximum_planets_per_coord
    );

    // ***************************** FAIL CASES *****************************

    /* Claim task that is not complete as owner */
    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(arrived_nft_info.clone()),
        vec![],
        &[],
    );

    let _ = setup_contract(deps.as_mut());
    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &non_completed_task);
    let result = execute(
        deps.as_mut(),
        _env.clone(),
        mock_info(&owner, &[]),
        msg.clone(),
    );

    assert_eq!(result.is_err(), true);
    assert_eq!(
        result.unwrap_err(),
        StdError::generic_err("Task is still in progress")
    );

    /* Claim task that is complete as non-owner */

    let mut deps = mock_dependencies_custom(
        Some(non_owner.to_string()),
        Some(DEFAULT_RAND),
        Some(arrived_nft_info.clone()),
        vec![],
        &[],
    );

    let _ = setup_contract(deps.as_mut());
    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &completed_task);
    let result = execute(
        deps.as_mut(),
        _env.clone(),
        mock_info(&owner, &[]),
        msg.clone(),
    );

    assert_eq!(result.is_err(), true);
    assert_eq!(
        result.unwrap_err(),
        StdError::generic_err("Wallet does not own NFT")
    );

    /* Claim task that does not exist for nft */

    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(arrived_nft_info.clone()),
        vec![],
        &[],
    );

    let _ = setup_contract(deps.as_mut());
    let result = execute(
        deps.as_mut(),
        _env.clone(),
        mock_info(&owner, &[]),
        msg.clone(),
    );

    assert_eq!(result.is_err(), true);
    assert_eq!(
        result.unwrap_err(),
        StdError::generic_err("No task in progress.")
    );

    /* Claim task that is complete for moving nft */

    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(moving_nft_info.clone()),
        vec![],
        &[],
    );

    let _ = setup_contract(deps.as_mut());
    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &completed_task);
    let result = execute(
        deps.as_mut(),
        _env.clone(),
        mock_info(&owner, &[]),
        msg.clone(),
    );

    assert_eq!(result.is_err(), true);
    assert_eq!(
        result.unwrap_err(),
        StdError::generic_err("Cannot complete a task for moving nft.")
    );

    /* Claim task that is complete as owner but is expired */

    let mut deps = mock_dependencies_custom(
        Some(owner.to_string()),
        Some(DEFAULT_RAND),
        Some(arrived_nft_info.clone()),
        vec![],
        &[],
    );

    let _ = setup_contract(deps.as_mut());
    let _ = TASK_REPOSITORY.save_task(deps.as_mut().storage, &completed_task_expired);
    let result = execute(
        deps.as_mut(),
        _env.clone(),
        mock_info(&owner, &[]),
        msg.clone(),
    );

    assert_eq!(result.is_ok(), true);
    let result = fetch_all_planets_for_coordinate(
        deps.as_mut().storage,
        &completed_task_expired.coordinates,
        None,
        None,
    )
    .unwrap();
    assert_eq!(result.len(), 0)
}
