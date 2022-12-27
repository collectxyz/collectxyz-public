use super::*;
use std::collections::HashMap;

use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{from_binary, Addr, Coin, DepsMut, Response, StdError, Timestamp, Uint128};

use collectxyz::nft::{Coordinates, XyzExtension, XyzTokenInfo};
use collectxyz_planet_metaverse::util::{burn_resource, mint_resource};

use crate::contract;
use crate::mock_querier::mock_dependencies_custom;
use crate::msg::{ExecuteMsg, InstantiateMsg, ListingsResponse, QueryMsg};
use crate::state::{Config, Listing, Resource};

const OWNER: &str = "owner";
const NONOWNER: &str = "nonowner";

fn initial_xyz_balances() -> HashMap<String, XyzTokenInfo> {
    HashMap::from([
        (
            "xyz #1".to_string(),
            XyzTokenInfo {
                owner: Addr::unchecked(OWNER),
                approvals: vec![],
                name: "xyz #1".to_string(),
                description: "".to_string(),
                image: None,
                extension: XyzExtension {
                    coordinates: Coordinates { x: 1, y: 1, z: 1 },
                    arrival: Timestamp::from_seconds(0),
                    prev_coordinates: None,
                },
            },
        ),
        (
            "xyz #2".to_string(),
            XyzTokenInfo {
                owner: Addr::unchecked(NONOWNER),
                approvals: vec![],
                name: "xyz #2".to_string(),
                description: "".to_string(),
                image: None,
                extension: XyzExtension {
                    coordinates: Coordinates { x: 2, y: 2, z: 2 },
                    arrival: Timestamp::from_seconds(10000),
                    prev_coordinates: None,
                },
            },
        ),
    ])
}

fn action_fee() -> Coin {
    Coin::new(1, "uusd")
}

fn mock_config() -> Config {
    Config {
        listing_expiry_seconds: 3,
        listing_pending_seconds: 1,
        listing_deposit_percent: 5,
        allowed_listing_prices: vec![Uint128::new(10), Uint128::new(100), Uint128::new(1000)],
        make_listing_fee: action_fee(),
        take_listing_fee: action_fee(),
        xyz_nft_contract: Addr::unchecked("xyz-nft-contract"),
        rock_contract: Addr::unchecked("rock-contract"),
        ice_contract: Addr::unchecked("ice-contract"),
        metal_contract: Addr::unchecked("metal-contract"),
        gas_contract: Addr::unchecked("gas-contract"),
        water_contract: Addr::unchecked("water-contract"),
        gem_contract: Addr::unchecked("gem-contract"),
        life_contract: Addr::unchecked("life-contract"),
    }
}

fn setup_contract(deps: DepsMut) {
    let msg = InstantiateMsg {
        config: mock_config(),
    };
    let res = contract::instantiate(deps, mock_env(), mock_info(OWNER, &[]), msg).unwrap();
    assert_eq!(res.messages.len(), 0);
}

#[test]
fn test_make_listing() {
    let xyz_balances = initial_xyz_balances();
    let mut deps = mock_dependencies_custom(xyz_balances, &[]);
    setup_contract(deps.as_mut());

    let xyz_id = "xyz #1".to_string();
    let price_rmi = Uint128::new(100);
    let deposit_rmi_denom = "xyzICE".to_string();

    // valid listing
    let res = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[action_fee()]),
        ExecuteMsg::MakeListing {
            lister_xyz_id: xyz_id.clone(),
            price_rmi: price_rmi.into(),
            deposit_rmi_denom: deposit_rmi_denom.clone(),
            resources: vec![
                Resource {
                    id: "xyzLIFE".to_string(),
                    amount: Uint128::new(123000000),
                },
                Resource {
                    id: "xyzGAS".to_string(),
                    amount: Uint128::new(321000000),
                },
            ],
        },
    )
    .unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_messages(vec![
                // Burn resource bundle from lister_xyz_id
                burn_resource(
                    OWNER.to_string(),
                    xyz_id.clone(),
                    "life-contract".to_string(),
                    Uint128::new(123000000)
                )
                .unwrap(),
                burn_resource(
                    OWNER.to_string(),
                    xyz_id.clone(),
                    "gas-contract".to_string(),
                    Uint128::new(321000000)
                )
                .unwrap(),
                // RMI deposit
                burn_resource(
                    OWNER.to_string(),
                    xyz_id.clone(),
                    "ice-contract".to_string(),
                    Uint128::new(5)
                )
                .unwrap()
            ])
            .add_attribute("method", "execute")
            .add_attribute("action", "make_listing")
            .add_attribute("listing", "{\"listing_id\":1,\"lister_xyz_id\":\"xyz #1\",\"price_rmi\":\"100\",\"deposit_rmi_denom\":\"xyzICE\",\"deposit_rmi_amount\":\"5\",\"created_at\":\"1571797419879305533\",\"active_at\":\"1571797420879305533\",\"expired_at\":\"1571797422879305533\",\"resources\":[{\"id\":\"xyzLIFE\",\"amount\":\"123000000\"},{\"id\":\"xyzGAS\",\"amount\":\"321000000\"}]}")
    );

    // check created listing has expected info
    let res: Listing = from_binary(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ListingInfo { listing_id: 1 },
        )
        .unwrap(),
    )
    .unwrap();

    let block_time = mock_env().block.time;
    assert_eq!(
        res,
        Listing {
            listing_id: 1,
            created_at: block_time,
            active_at: block_time.plus_seconds(1),
            expired_at: block_time.plus_seconds(3),
            lister_xyz_id: xyz_id.clone(),
            price_rmi: price_rmi.clone(),
            deposit_rmi_amount: Uint128::new(5),
            deposit_rmi_denom: deposit_rmi_denom.clone(),
            resources: vec![
                Resource {
                    id: "xyzLIFE".to_string(),
                    amount: Uint128::new(123000000),
                },
                Resource {
                    id: "xyzGAS".to_string(),
                    amount: Uint128::new(321000000),
                },
            ],
        }
    );

    // xyz that sender doesn't own
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(NONOWNER, &[action_fee()]),
        ExecuteMsg::MakeListing {
            lister_xyz_id: "xyz #1".to_string(),
            price_rmi: price_rmi.clone(),
            deposit_rmi_denom: deposit_rmi_denom.clone(),
            resources: vec![],
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // insufficient make_listing_fee sent
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        ExecuteMsg::MakeListing {
            lister_xyz_id: "xyz #1".to_string(),
            price_rmi: price_rmi.clone(),
            deposit_rmi_denom: deposit_rmi_denom.clone(),
            resources: vec![],
        },
    )
    .unwrap_err();
    assert_eq!(
        err,
        ContractError::Std(StdError::generic_err("insufficient funds sent"))
    );

    // invalid listing price
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[action_fee()]),
        ExecuteMsg::MakeListing {
            lister_xyz_id: "xyz #1".to_string(),
            price_rmi: Uint128::new(123000000),
            deposit_rmi_denom: deposit_rmi_denom.clone(),
            resources: vec![],
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::InvalidListingPrice {});

    // invalid deposit_rmi_denom
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[action_fee()]),
        ExecuteMsg::MakeListing {
            lister_xyz_id: xyz_id.clone(),
            price_rmi: price_rmi.clone(),
            deposit_rmi_denom: "foobar".to_string(),
            resources: vec![],
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::InvalidResourceId("foobar".to_string()));

    // empty resource bundle
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[action_fee()]),
        ExecuteMsg::MakeListing {
            lister_xyz_id: xyz_id.clone(),
            price_rmi: price_rmi.clone(),
            deposit_rmi_denom: deposit_rmi_denom.clone(),
            resources: vec![],
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::EmptyResourceBundle {});

    // invalid resource bundle - non-integer price
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[action_fee()]),
        ExecuteMsg::MakeListing {
            lister_xyz_id: xyz_id.clone(),
            price_rmi: price_rmi.clone(),
            deposit_rmi_denom: deposit_rmi_denom.clone(),
            resources: vec![Resource {
                id: "xyzICE".to_string(),
                amount: Uint128::new(123000001),
            }],
        },
    )
    .unwrap_err();
    assert_eq!(
        err,
        ContractError::PartialResourceAmount(Uint128::new(123000001))
    );

    // invalid resource bundle - bad resource id
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[action_fee()]),
        ExecuteMsg::MakeListing {
            lister_xyz_id: xyz_id.clone(),
            price_rmi: price_rmi.clone(),
            deposit_rmi_denom: deposit_rmi_denom.clone(),
            resources: vec![Resource {
                id: "barfoo".to_string(),
                amount: Uint128::new(123000000),
            }],
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::InvalidResourceId("barfoo".to_string()));

    // invalid resource bundle - duplicate resource ids
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[action_fee()]),
        ExecuteMsg::MakeListing {
            lister_xyz_id: xyz_id.clone(),
            price_rmi: price_rmi.clone(),
            deposit_rmi_denom: deposit_rmi_denom.clone(),
            resources: vec![
                Resource {
                    id: "xyzLIFE".to_string(),
                    amount: Uint128::new(123000000),
                },
                Resource {
                    id: "xyzGAS".to_string(),
                    amount: Uint128::new(123000000),
                },
                Resource {
                    id: "xyzLIFE".to_string(),
                    amount: Uint128::new(123000000),
                },
            ],
        },
    )
    .unwrap_err();
    assert_eq!(
        err,
        ContractError::DuplicateResourceId("xyzLIFE".to_string())
    );
}

#[test]
fn revoke_listing() {
    let xyz_balances = initial_xyz_balances();
    let mut deps = mock_dependencies_custom(xyz_balances.clone(), &[]);
    setup_contract(deps.as_mut());

    // can't revoke a non-existent listing
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        ExecuteMsg::RevokeListing { listing_id: 1 },
    )
    .unwrap_err();
    assert_eq!(
        err,
        ContractError::Std(StdError::not_found(
            "collectxyz_marketplace_contract::state::Listing"
        ))
    );

    let xyz_id = "xyz #1".to_string();

    // create an listing
    let _ = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[action_fee()]),
        ExecuteMsg::MakeListing {
            lister_xyz_id: xyz_id.clone(),
            price_rmi: Uint128::new(10),
            deposit_rmi_denom: "xyzICE".to_string(),
            resources: vec![
                Resource {
                    id: "xyzGEM".to_string(),
                    amount: Uint128::new(123000000),
                },
                Resource {
                    id: "xyzWATER".to_string(),
                    amount: Uint128::new(321000000),
                },
            ],
        },
    )
    .unwrap();

    // "transfer" xyz to new owner
    let mut new_xyz_balances = xyz_balances.clone();
    let transferred_xyz = new_xyz_balances.get_mut("xyz #1").unwrap();
    transferred_xyz.owner = Addr::unchecked(NONOWNER);
    deps.querier.update_xyz_balances(new_xyz_balances);

    // can't revoke listing if you don't own its lister xyz
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        ExecuteMsg::RevokeListing { listing_id: 1 },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // revoke listing
    let res = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(NONOWNER, &[]),
        ExecuteMsg::RevokeListing { listing_id: 1 },
    )
    .unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_messages(vec![
                // Mint resource bundle to lister_xyz_id
                mint_resource(
                    xyz_id.clone(),
                    "gem-contract".to_string(),
                    Uint128::new(123000000)
                )
                .unwrap(),
                mint_resource(
                    xyz_id.clone(),
                    "water-contract".to_string(),
                    Uint128::new(321000000)
                )
                .unwrap()
            ])
            .add_attribute("method", "execute")
            .add_attribute("action", "revoke_listing")
            .add_attribute("listing", "{\"listing_id\":1,\"lister_xyz_id\":\"xyz #1\",\"price_rmi\":\"10\",\"deposit_rmi_denom\":\"xyzICE\",\"deposit_rmi_amount\":\"0\",\"created_at\":\"1571797419879305533\",\"active_at\":\"1571797420879305533\",\"expired_at\":\"1571797422879305533\",\"resources\":[{\"id\":\"xyzGEM\",\"amount\":\"123000000\"},{\"id\":\"xyzWATER\",\"amount\":\"321000000\"}]}")
    )
}

#[test]
fn test_take_listing() {
    let xyz_balances = initial_xyz_balances();
    let mut deps = mock_dependencies_custom(xyz_balances.clone(), &[]);
    setup_contract(deps.as_mut());

    let lister_xyz_id = "xyz #1".to_string();
    let deposit_rmi_denom = "xyzICE".to_string();
    let price_rmi = Uint128::new(1000);
    let listing_id = 1;
    let taker_xyz_id = "xyz #2".to_string();
    let taker_rmi_denom = "xyzMETAL".to_string();

    // create a listing
    let _ = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[action_fee()]),
        ExecuteMsg::MakeListing {
            lister_xyz_id: lister_xyz_id.clone(),
            price_rmi: price_rmi.clone(),
            deposit_rmi_denom: deposit_rmi_denom.clone(),
            resources: vec![
                Resource {
                    id: "xyzROCK".to_string(),
                    amount: Uint128::new(123000000),
                },
                Resource {
                    id: "xyzGAS".to_string(),
                    amount: Uint128::new(321000000),
                },
                Resource {
                    id: "xyzGEM".to_string(),
                    amount: Uint128::new(1000000),
                },
            ],
        },
    )
    .unwrap();

    // can't take listing for xyz you don't own
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[action_fee()]),
        ExecuteMsg::TakeListing {
            listing_id: listing_id.clone(),
            taker_xyz_id: taker_xyz_id.clone(),
            rmi_denom: taker_rmi_denom.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // can't take listing without sufficient listing fee
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(NONOWNER, &[]),
        ExecuteMsg::TakeListing {
            listing_id: listing_id.clone(),
            taker_xyz_id: taker_xyz_id.clone(),
            rmi_denom: taker_rmi_denom.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(
        err,
        ContractError::Std(StdError::generic_err("insufficient funds sent"))
    );

    // can't take a pending listing
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(NONOWNER, &[action_fee()]),
        ExecuteMsg::TakeListing {
            listing_id: listing_id.clone(),
            taker_xyz_id: taker_xyz_id.clone(),
            rmi_denom: taker_rmi_denom.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::InactiveListing {});

    // can't take an expired listing
    let mut env = mock_env();
    env.block.time = env.block.time.plus_seconds(100);
    let err = contract::execute(
        deps.as_mut(),
        env,
        mock_info(NONOWNER, &[action_fee()]),
        ExecuteMsg::TakeListing {
            listing_id: listing_id.clone(),
            taker_xyz_id: taker_xyz_id.clone(),
            rmi_denom: taker_rmi_denom.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::InactiveListing {});

    // set up env where listing will be active
    let mut active_env = mock_env();
    active_env.block.time = active_env.block.time.plus_seconds(1);

    // can't take own listing
    let err = contract::execute(
        deps.as_mut(),
        active_env.clone(),
        mock_info(OWNER, &[action_fee()]),
        ExecuteMsg::TakeListing {
            listing_id: listing_id.clone(),
            taker_xyz_id: lister_xyz_id.clone(),
            rmi_denom: taker_rmi_denom.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::CantTakeOwnListing {});

    // can take an active listing
    let res = contract::execute(
        deps.as_mut(),
        active_env.clone(),
        mock_info(NONOWNER, &[action_fee()]),
        ExecuteMsg::TakeListing {
            listing_id: listing_id.clone(),
            taker_xyz_id: taker_xyz_id.clone(),
            rmi_denom: taker_rmi_denom.clone(),
        },
    )
    .unwrap();

    assert_eq!(
        res,
        Response::new()
            .add_messages(vec![
                // Mint price in taker's RMI denom to lister_xyz_id
                mint_resource(
                    lister_xyz_id.clone(),
                    "metal-contract".to_string(),
                    price_rmi
                )
                .unwrap(),
                // Burn price in taker's RMI denom from taker_xyz_id
                burn_resource(
                    NONOWNER.to_string(),
                    taker_xyz_id.clone(),
                    "metal-contract".to_string(),
                    price_rmi
                )
                .unwrap(),
                // Return RMI deposit
                mint_resource(
                    lister_xyz_id.clone(),
                    "ice-contract".to_string(),
                    Uint128::new(50) // 5% of 1000
                )
                .unwrap(),
                // Mint resource bundle to taker_xyz_id
                mint_resource(
                    taker_xyz_id.clone(),
                    "rock-contract".to_string(),
                    Uint128::new(123000000)
                )
                .unwrap(),
                mint_resource(
                    taker_xyz_id.clone(),
                    "gas-contract".to_string(),
                    Uint128::new(321000000)
                )
                .unwrap(),
                mint_resource(
                    taker_xyz_id.clone(),
                    "gem-contract".to_string(),
                    Uint128::new(1000000)
                )
                .unwrap(),
            ])
            .add_attribute("method", "execute")
            .add_attribute("action", "take_listing")
            .add_attribute("taker_xyz_id", taker_xyz_id)
            .add_attribute("listing", "{\"listing_id\":1,\"lister_xyz_id\":\"xyz #1\",\"price_rmi\":\"1000\",\"deposit_rmi_denom\":\"xyzICE\",\"deposit_rmi_amount\":\"50\",\"created_at\":\"1571797419879305533\",\"active_at\":\"1571797420879305533\",\"expired_at\":\"1571797422879305533\",\"resources\":[{\"id\":\"xyzROCK\",\"amount\":\"123000000\"},{\"id\":\"xyzGAS\",\"amount\":\"321000000\"},{\"id\":\"xyzGEM\",\"amount\":\"1000000\"}]}")
    );
}

#[test]
fn test_query_listings() {
    let xyz_balances = initial_xyz_balances();
    let mut deps = mock_dependencies_custom(xyz_balances.clone(), &[]);
    setup_contract(deps.as_mut());

    let owner_xyz_id = "xyz #1".to_string();
    let nonowner_xyz_id = "xyz #2".to_string();
    let deposit_rmi_denom = "xyzICE".to_string();

    // insert a bunch of listings
    let pending_env = mock_env();
    let mut active_env = mock_env();
    active_env.block.time = active_env.block.time.minus_seconds(1);
    let mut expired_env = mock_env();
    expired_env.block.time = expired_env.block.time.minus_seconds(3);
    for (sender, env, msg) in vec![
        (
            OWNER,
            pending_env,
            ExecuteMsg::MakeListing {
                lister_xyz_id: owner_xyz_id.clone(),
                deposit_rmi_denom: deposit_rmi_denom.to_string(),
                price_rmi: Uint128::new(10),
                resources: vec![
                    Resource {
                        id: "xyzROCK".to_string(),
                        amount: Uint128::new(1000000),
                    },
                    Resource {
                        id: "xyzGAS".to_string(),
                        amount: Uint128::new(2000000),
                    },
                    Resource {
                        id: "xyzGEM".to_string(),
                        amount: Uint128::new(3000000),
                    },
                ],
            },
        ),
        (
            OWNER,
            active_env,
            ExecuteMsg::MakeListing {
                lister_xyz_id: owner_xyz_id.clone(),
                deposit_rmi_denom: deposit_rmi_denom.to_string(),
                price_rmi: Uint128::new(100),
                resources: vec![
                    Resource {
                        id: "xyzICE".to_string(),
                        amount: Uint128::new(1000000),
                    },
                    Resource {
                        id: "xyzGEM".to_string(),
                        amount: Uint128::new(2000000),
                    },
                ],
            },
        ),
        (
            NONOWNER,
            expired_env,
            ExecuteMsg::MakeListing {
                lister_xyz_id: nonowner_xyz_id.clone(),
                deposit_rmi_denom: deposit_rmi_denom.to_string(),
                price_rmi: Uint128::new(1000),
                resources: vec![
                    Resource {
                        id: "xyzROCK".to_string(),
                        amount: Uint128::new(1000000),
                    },
                    Resource {
                        id: "xyzICE".to_string(),
                        amount: Uint128::new(1000000),
                    },
                    Resource {
                        id: "xyzGAS".to_string(),
                        amount: Uint128::new(2000000),
                    },
                ],
            },
        ),
    ]
    .iter()
    {
        let _ = contract::execute(
            deps.as_mut(),
            env.clone(),
            mock_info(sender, &[action_fee()]),
            msg.clone(),
        )
        .unwrap();
    }

    let get_listing_ids = |response: ListingsResponse| {
        response
            .listings
            .iter()
            .map(|o| o.listing_id)
            .collect::<Vec<u64>>()
    };

    // query active listings
    let res: ListingsResponse = from_binary(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Listings {
                lister_xyz_id: None,
                prices: None,
                resources: None,
                include_inactive: None,
                ascending: None,
                start_after: None,
                limit: None,
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(get_listing_ids(res), vec![2]);

    // query all listings
    let res: ListingsResponse = from_binary(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Listings {
                lister_xyz_id: None,
                prices: None,
                resources: None,
                include_inactive: Some(true),
                ascending: None,
                start_after: None,
                limit: None,
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(get_listing_ids(res), vec![3, 2, 1]);

    // query all listings ascending
    let res: ListingsResponse = from_binary(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Listings {
                lister_xyz_id: None,
                prices: None,
                resources: None,
                include_inactive: Some(true),
                ascending: Some(true),
                start_after: None,
                limit: None,
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(get_listing_ids(res), vec![1, 2, 3]);

    // filter by price
    let res: ListingsResponse = from_binary(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Listings {
                lister_xyz_id: None,
                prices: Some(vec![Uint128::new(10), Uint128::new(100)]),
                resources: None,
                include_inactive: Some(true),
                ascending: None,
                start_after: None,
                limit: None,
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(get_listing_ids(res), vec![2, 1]);

    // filter by resource
    let res: ListingsResponse = from_binary(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Listings {
                lister_xyz_id: None,
                prices: None,
                resources: Some(vec!["xyzROCK".to_string(), "xyzGAS".to_string()]),
                include_inactive: Some(true),
                ascending: None,
                start_after: None,
                limit: None,
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(get_listing_ids(res), vec![3, 1]);

    // filter by resource and price
    let res: ListingsResponse = from_binary(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Listings {
                lister_xyz_id: None,
                prices: Some(vec![Uint128::new(10), Uint128::new(100)]),
                resources: Some(vec!["xyzROCK".to_string(), "xyzGAS".to_string()]),
                include_inactive: Some(true),
                ascending: None,
                start_after: None,
                limit: None,
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(get_listing_ids(res), vec![1]);

    // filter by resource and lister
    let res: ListingsResponse = from_binary(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Listings {
                lister_xyz_id: Some(owner_xyz_id.clone()),
                prices: None,
                resources: Some(vec!["xyzROCK".to_string(), "xyzGAS".to_string()]),
                include_inactive: Some(true),
                ascending: None,
                start_after: None,
                limit: None,
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(get_listing_ids(res), vec![1]);

    // start_after descending
    let res: ListingsResponse = from_binary(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Listings {
                lister_xyz_id: None,
                prices: None,
                resources: None,
                include_inactive: Some(true),
                ascending: None,
                start_after: Some(3),
                limit: None,
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(get_listing_ids(res), vec![2, 1]);

    // start_after ascending
    let res: ListingsResponse = from_binary(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Listings {
                lister_xyz_id: None,
                prices: None,
                resources: None,
                include_inactive: Some(true),
                ascending: Some(true),
                start_after: Some(1),
                limit: None,
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(get_listing_ids(res), vec![2, 3]);

    // limit with filter
    let res: ListingsResponse = from_binary(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Listings {
                lister_xyz_id: None,
                prices: None,
                resources: Some(vec!["xyzGAS".to_string()]),
                include_inactive: Some(true),
                ascending: None,
                start_after: None,
                limit: Some(2),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(get_listing_ids(res), vec![3, 1]);
}
