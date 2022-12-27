#![cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{Addr, DepsMut, Env, Timestamp, Uint128};

use crate::error::ContractError;
use crate::execute as ExecHandler;
use crate::mock_querier::mock_dependencies_custom;
use crate::msg::InstantiateMsg;
use crate::query as QueryHandler;
use crate::state::{Config, Seed};

const OWNER: &str = "owner";
const NONOWNER: &str = "nonowner";
const BASE_TS_SECS: u64 = 1571797419;
const TIME_SLOT_SECS: u64 = 1;
const CW20_CONTRACT: &str = "cw20-contract-addr";
const RESOURCE_GATHERING_CONTRACT: &str = "resource-gathering-contract-addr";

// SHA512 hash of the concatenated base64 encodings of the two queries for the seeds
// used in setup_contract below plus the timestamp at BASE_TS_SECS. The mock querier just
// echos back seed queries, so this is the hash result we expect when querying seeds
// for randomness.
const SEED_HASH: &[u8] = &[
    90, 110, 74, 67, 234, 167, 221, 41, 135, 253, 217, 0, 27, 157, 136, 30, 188, 154, 145, 200,
    106, 45, 14, 41, 182, 246, 201, 59, 222, 161, 72, 59, 52, 53, 221, 151, 153, 107, 172, 42, 148,
    39, 39, 120, 248, 173, 15, 23, 206, 115, 151, 176, 154, 204, 23, 66, 161, 157, 77, 220, 192,
    31, 15, 144,
];

// Set up a rand contract with two seeds, 1 second time slots, and 3 second time slot expiry.
fn setup_contract(deps: DepsMut) -> InstantiateMsg {
    let msg = InstantiateMsg {
        seeds: vec![
            Seed {
                contract_addr: Addr::unchecked("seed-1"),
                query: r#"{"some": "query"}"#.to_string(),
            },
            Seed {
                contract_addr: Addr::unchecked("seed-2"),
                query: r#"{"another": "query"}"#.to_string(),
            },
        ],
        time_slot_nanos: Timestamp::from_seconds(TIME_SLOT_SECS).nanos(),
        expiry_nanos: Timestamp::from_seconds(3).nanos(),
        cw20_contract: Addr::unchecked(CW20_CONTRACT),
        minting_addresses: vec![Addr::unchecked(RESOURCE_GATHERING_CONTRACT)],
    };
    let _ = ExecHandler::instantiate(deps, mock_env(), mock_info(OWNER, &[]), msg.clone()).unwrap();

    msg
}

fn mock_env_block_time(ts: Timestamp) -> Env {
    let mut env = mock_env();
    env.block.time = ts;
    env
}

#[test]
fn instantiate() {
    let mut deps = mock_dependencies(&[]);
    let msg = setup_contract(deps.as_mut());

    // query to make sure the contract's config was updated
    let result = QueryHandler::query_config(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(msg.seeds, result.config.seeds);
    assert_eq!(msg.time_slot_nanos, result.config.time_slot_nanos);
    assert_eq!(msg.cw20_contract, result.config.cw20_contract);
    assert_eq!(msg.minting_addresses, result.config.minting_addresses);
}

#[test]
fn update_rand() {
    let mut deps = mock_dependencies_custom(&[]);
    setup_contract(deps.as_mut());

    let base_ts = Timestamp::from_seconds(BASE_TS_SECS);
    let env = mock_env_block_time(base_ts);
    let res =
        ExecHandler::execute_update_rand(deps.as_mut(), env, mock_info(NONOWNER, &[])).unwrap();
    println!("{:#?}", res);
    // check that a bonus was awarded
    assert_eq!(res.messages.len(), 1);

    let latest_rand = QueryHandler::query_latest_rand(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(latest_rand.slot, base_ts.nanos());
    assert_eq!(latest_rand.rand, SEED_HASH);

    // re-submit for the same time slot at a slightly later timestamp
    let shifted_ts = base_ts.plus_nanos(100);
    let env = mock_env_block_time(shifted_ts);
    let err =
        ExecHandler::execute_update_rand(deps.as_mut(), env, mock_info(NONOWNER, &[])).unwrap_err();
    assert_eq!(err, ContractError::BonusClaimed {});

    // later re-submissions don't update the stored hash
    let latest_rand = QueryHandler::query_latest_rand(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(latest_rand.slot, base_ts.nanos());
    assert_eq!(latest_rand.rand, SEED_HASH);

    // submissions at a new time slot add new rand and award a bonus
    let new_slot_ts = base_ts.plus_seconds(TIME_SLOT_SECS);
    let env = mock_env_block_time(new_slot_ts);
    let res =
        ExecHandler::execute_update_rand(deps.as_mut(), env, mock_info(NONOWNER, &[])).unwrap();
    assert_eq!(res.messages.len(), 1);
    let latest_rand = QueryHandler::query_latest_rand(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(latest_rand.slot, new_slot_ts.nanos());
    assert_ne!(latest_rand.rand, SEED_HASH);
}

#[test]
fn timestamp_rand() {
    let mut deps = mock_dependencies_custom(&[]);
    setup_contract(deps.as_mut());

    // Add initial randomness
    let base_ts = Timestamp::from_seconds(BASE_TS_SECS);
    let env = mock_env_block_time(base_ts);
    let _ = ExecHandler::execute_update_rand(deps.as_mut(), env, mock_info(NONOWNER, &[])).unwrap();

    // Two different timestamps in the same time slot yield same randomness
    for ts in vec![base_ts.plus_nanos(123456), base_ts.plus_nanos(654321)] {
        let res = QueryHandler::query_timestamp_rand(deps.as_ref(), mock_env(), ts).unwrap();
        assert_eq!(res.rand, SEED_HASH);
        assert_eq!(res.slot, base_ts.nanos());
    }

    // Timestamp outside of time slot defaults to available randomness
    let shifted_ts = base_ts.plus_seconds(1000);
    let res = QueryHandler::query_timestamp_rand(
        deps.as_ref(),
        mock_env(),
        shifted_ts.plus_nanos(123456),
    )
    .unwrap();
    assert_eq!(res.rand, SEED_HASH);
    assert_eq!(res.slot, shifted_ts.nanos());

    // Update timestamp at new time slot
    let env = mock_env_block_time(shifted_ts);
    let _ = ExecHandler::execute_update_rand(deps.as_mut(), env, mock_info(NONOWNER, &[])).unwrap();

    // Get rand at new time slot
    let res = QueryHandler::query_timestamp_rand(
        deps.as_ref(),
        mock_env(),
        shifted_ts.plus_nanos(123456),
    )
    .unwrap();
    assert_ne!(res.rand, SEED_HASH);
    assert_eq!(res.slot, shifted_ts.nanos());

    // Check that old time slot DID NOT expire, and query for that time slot to make sure.
    let res = QueryHandler::query_timestamp_rand(deps.as_ref(), mock_env(), base_ts).unwrap();
    assert_eq!(res.rand, SEED_HASH);
}

#[test]
fn update_config() {
    let mut deps = mock_dependencies_custom(&[]);
    setup_contract(deps.as_mut());

    let new_config = Config {
        seeds: vec![],
        time_slot_nanos: Timestamp::from_seconds(123212321).nanos(),
        expiry_nanos: Timestamp::from_seconds(9999).nanos(),
        cw20_contract: Addr::unchecked("foo bar foo bar"),
        minting_addresses: vec![],
    };

    // non-owner can't update config
    let err = ExecHandler::execute_update_config(
        deps.as_mut(),
        mock_env(),
        mock_info(NONOWNER, &[]),
        new_config.clone(),
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    let res = QueryHandler::query_config(deps.as_ref(), mock_env()).unwrap();
    assert_ne!(res.config, new_config);

    // owner can update config
    let _ = ExecHandler::execute_update_config(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        new_config.clone(),
    )
    .unwrap();

    let res = QueryHandler::query_config(deps.as_ref(), mock_env()).unwrap();
    assert_eq!(res.config, new_config);
}

#[test]
fn mint_bonus_token() {
    let mut deps = mock_dependencies_custom(&[]);
    setup_contract(deps.as_mut());

    // unauthorized if not in list
    let err = ExecHandler::execute_mint_bonus_token(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        OWNER.to_string(),
        Uint128::new(1000000),
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // authorized if in list
    let res = ExecHandler::execute_mint_bonus_token(
        deps.as_mut(),
        mock_env(),
        mock_info(RESOURCE_GATHERING_CONTRACT, &[]),
        OWNER.to_string(),
        Uint128::new(1000000),
    )
    .unwrap();

    assert_eq!(res.messages.len(), 1);
}
