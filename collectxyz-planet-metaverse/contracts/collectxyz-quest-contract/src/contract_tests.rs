use std::collections::HashMap;

use collectxyz::nft::{Coordinates, XyzExtension, XyzTokenInfo};
use collectxyz_planet_metaverse::experience::XyzExperienceMintInfo;
use collectxyz_planet_metaverse::mock_querier::{
    EXPERIENCE_CONTRACT_ADDRESS, NFT_CONTRACT_ADDRESS, RANDOM_CONTRACT_ADDRESS,
};
use collectxyz_planet_metaverse::util::burn_resource;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{
    from_binary, Addr, BankMsg, Coin, DepsMut, Response, Timestamp, Uint128, Uint64,
};

use crate::contract;
use crate::error::ContractError;
use crate::mock_querier::mock_dependencies_custom;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    CompleteObjective, Completed, Config, Goal, GoalInfo, Objective, RequiredResource,
    ResourceConfig, CONFIG,
};

const OWNER: &str = "owner";
const NONOWNER: &str = "nonowner";

const MOCK_QUEST_NAME: &str = "mockQuestName";
const MOCK_QUEST_TIMESTAMP: u64 = 1639933720u64;
const FIVE_DAYS: u64 = 60 * 60 * 24 * 5;
const ONE_DAY: u64 = 60 * 60 * 24;

fn mock_resource_config() -> ResourceConfig {
    ResourceConfig {
        rock_contract: Addr::unchecked("trrock_contract"),
        ice_contract: Addr::unchecked("trice_contract"),
        metal_contract: Addr::unchecked("trmetal_contract"),
        gas_contract: Addr::unchecked("trgas_contract"),
        water_contract: Addr::unchecked("trwater_contract"),
        gem_contract: Addr::unchecked("trgem_contract"),
        life_contract: Addr::unchecked("trlife_contract"),
    }
}

fn mock_config(goal_info: Option<Vec<GoalInfo>>, objectives: Option<Vec<Objective>>) -> Config {
    let quest_start_time = Timestamp::from_seconds(MOCK_QUEST_TIMESTAMP.clone());

    let possible_goal_info = goal_info.unwrap_or(vec![
        GoalInfo {
            name: "goal1".to_string(),
            xp_amount: Uint128::new(1),
            rock_weighting: Some(Uint64::new(50)),
            ice_weighting: None,
            metal_weighting: Some(Uint64::new(50)),
            gas_weighting: None,
            water_weighting: None,
            gem_weighting: None,
            life_weighting: None,
        },
        GoalInfo {
            name: "goal2".to_string(),
            xp_amount: Uint128::new(1),
            rock_weighting: Some(Uint64::new(80)),
            ice_weighting: None,
            metal_weighting: None,
            gas_weighting: Some(Uint64::new(20)),
            water_weighting: None,
            gem_weighting: None,
            life_weighting: None,
        },
        GoalInfo {
            name: "goal3".to_string(),
            xp_amount: Uint128::new(1),
            rock_weighting: Some(Uint64::new(80)),
            ice_weighting: None,
            metal_weighting: None,
            gas_weighting: None,
            water_weighting: Some(Uint64::new(15)),
            gem_weighting: None,
            life_weighting: Some(Uint64::new(5)),
        },
    ]);

    let objectives = objectives.unwrap_or(vec![
        Objective {
            objective_id: 0,
            objective_start_time: quest_start_time.plus_seconds(ONE_DAY),
            duration: ONE_DAY,
            multiplier: 1000,
            goal: None,
            late_penalty: 0,
            possible_goals_info: vec![possible_goal_info[0].clone(), possible_goal_info[1].clone()],
            desc: None,
        },
        Objective {
            objective_id: 1,
            objective_start_time: quest_start_time.plus_seconds(ONE_DAY * 2),
            duration: ONE_DAY,
            multiplier: 5000,
            goal: None,
            late_penalty: 0,
            possible_goals_info: vec![possible_goal_info[0].clone(), possible_goal_info[1].clone(), possible_goal_info[2].clone()],
            desc: None,
        },
        Objective {
            objective_id: 2,
            objective_start_time: quest_start_time.plus_seconds(ONE_DAY * 3),
            duration: ONE_DAY,
            multiplier: 10000,
            goal: None,
            late_penalty: 0,
            possible_goals_info: vec![possible_goal_info[0].clone(), possible_goal_info[1].clone(), possible_goal_info[2].clone()],
            desc: None,
        },
    ]);

    Config {
        quest_name: MOCK_QUEST_NAME.to_string(),
        start_time: quest_start_time,
        quest_duration_seconds: FIVE_DAYS.into(),
        xp_contract: Addr::unchecked(EXPERIENCE_CONTRACT_ADDRESS),
        objectives: objectives,
        xyz_nft_contract: Addr::unchecked(NFT_CONTRACT_ADDRESS),
        randomness_contract: Addr::unchecked(RANDOM_CONTRACT_ADDRESS),
        resource_configs: mock_resource_config(),
    }
}

fn setup_contract(deps: DepsMut) {
    let msg = InstantiateMsg {
        config: mock_config(None, None),
    };
    let res = contract::instantiate(deps, mock_env(), mock_info(OWNER, &[]), msg).unwrap();
    assert_eq!(res.messages.len(), 0);
}

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
                    arrival: Timestamp::from_seconds(MOCK_QUEST_TIMESTAMP),
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
                    arrival: Timestamp::from_seconds(MOCK_QUEST_TIMESTAMP + 10 * FIVE_DAYS),
                    prev_coordinates: None,
                },
            },
        ),
    ])
}

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies(&[]);
    let mut config = mock_config(None, None);

    // valid config
    let _ = contract::instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        InstantiateMsg {
            config: config.clone(),
        },
    )
    .unwrap();

    // invalid quest start time
    let mut env = mock_env();
    env.block.time = config.start_time.plus_seconds(1);
    let err = contract::instantiate(
        deps.as_mut(),
        env,
        mock_info(OWNER, &[]),
        InstantiateMsg {
            config: config.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(
        err,
        ContractError::InvalidConfig(
            "start_time".to_string(),
            "start_time is in the past".to_string()
        )
    );

    // invalid objective start time - before quest start
    config.objectives[0].objective_start_time = config.start_time.minus_seconds(1);
    let err = contract::instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        InstantiateMsg {
            config: config.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(
        err,
        ContractError::InvalidConfig(
            "objectives".to_string(),
            "objective_start_time must be after Quest start_time".to_string()
        )
    );

    // invalid objective start time - start time after quest expiry
    config.objectives[0].objective_start_time = config.start_time.plus_seconds(FIVE_DAYS * 2);
    let err = contract::instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        InstantiateMsg {
            config: config.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(
        err,
        ContractError::InvalidConfig(
            "objectives".to_string(),
            "objective_start_time must be before Quest end_time".to_string()
        )
    );

    // reset objective_start_time to something valid
    config.objectives[0].objective_start_time = config.start_time.plus_seconds(1);

    // invalid objective start time - end time after quest expiry
    config.objectives[0].duration = FIVE_DAYS * 5;
    let err = contract::instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        InstantiateMsg {
            config: config.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(
        err,
        ContractError::InvalidConfig(
            "objectives".to_string(),
            "objective completion must be before Quest end_time".to_string()
        )
    );

    // reset objective duration to something valid
    config.objectives[0].duration = 1;

    // out-of-order objective id
    config.objectives[1].objective_id = config.objectives[0].objective_id;
    let err = contract::instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        InstantiateMsg {
            config: config.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(
        err,
        ContractError::InvalidConfig(
            "objectives".to_string(),
            "objective_id: got 0, but expected 1".to_string()
        )
    );
}

#[test]
fn test_complete_objective() {
    let xyz_balances = initial_xyz_balances();
    let mut deps = mock_dependencies_custom(xyz_balances, &[]);
    setup_contract(deps.as_mut());

    let xyz_id = "xyz #1".to_string();

    // Fail if sender is not XYZ owner
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(NONOWNER, &[]),
        ExecuteMsg::CompleteObjective {
            objective_id: 1,
            xyz_id: xyz_id.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // Fail for future objectives
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        ExecuteMsg::CompleteObjective {
            objective_id: 1,
            xyz_id: xyz_id.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::ObjectiveNotStarted {});

    // Fail if the quest has expired
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(MOCK_QUEST_TIMESTAMP).plus_seconds(FIVE_DAYS + 1);
    let err = contract::execute(
        deps.as_mut(),
        env.clone(),
        mock_info(OWNER, &[]),
        ExecuteMsg::CompleteObjective {
            objective_id: 1,
            xyz_id: xyz_id.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::ExpiredQuest {});

    // Fail if the objective doesn't exist
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        ExecuteMsg::CompleteObjective {
            objective_id: 123,
            xyz_id: xyz_id.clone(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::ObjectiveNotFound("123".to_string()));

    // Get available objectives
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(MOCK_QUEST_TIMESTAMP).plus_seconds(ONE_DAY * 2 + 1);
    let objectives = from_binary::<Vec<Objective>>(
        &contract::query(deps.as_ref(), env.clone(), QueryMsg::GetObjectives {}).unwrap(),
    )
    .unwrap();

    let resource_config = mock_resource_config();

    // Succeeds for all objectives and fails for inactive for both xyz
    // NOTE: xyz #2 is currently relocating
    for (owner, xyz_id) in vec![(OWNER, "xyz #1"), (NONOWNER, "xyz #2")].iter() {
        // make sure we're actually testing something
        let mut saw_success = false;
        let mut saw_failure = false;
        for objective in objectives.iter() {
            if let Some(goal) = objective.clone().goal {
                saw_success = true;
                let res = contract::execute(
                    deps.as_mut(),
                    env.clone(),
                    mock_info(owner, &[]),
                    ExecuteMsg::CompleteObjective {
                        objective_id: objective.objective_id,
                        xyz_id: xyz_id.to_string(),
                    },
                )
                .unwrap();

                // Since goal was loaded by querying objectives,
                // we enforce that the query_objectives and execute_complete_objective
                // handlers construct goals in the same way.
                let mut expected_messages: Vec<_> = goal
                    .required_resources
                    .iter()
                    .map(|resource| {
                        let contract_addr = resource_config
                            .clone()
                            .resource_addr(&resource.resource_id)
                            .unwrap();
                        burn_resource(
                            owner.to_string(),
                            xyz_id.to_string(),
                            contract_addr.to_string(),
                            resource.required_amount.checked_mul(1000000u128.into()).unwrap_or(Uint128::MAX),
                        )
                        .unwrap()
                    })
                    .collect();
                expected_messages.push(
                    XyzExperienceMintInfo {
                        complete_task_experience_amount: goal.xp_reward,
                        experience_contract_address: Addr::unchecked(EXPERIENCE_CONTRACT_ADDRESS.to_string())
                    }.mint_experince(xyz_id.to_string()).unwrap()
                );
                assert_eq!(
                    res,
                    Response::new()
                        .add_messages(expected_messages)
                        .add_attribute("method", "execute")
                        .add_attribute("action", "complete_objective")
                        .add_attribute(
                            "objective",
                            serde_json::to_string(&CompleteObjective {
                                xyz_id: xyz_id.to_string(),
                                completed_timestamp: env.block.time,
                                objective: objective.clone(),
                            })
                            .unwrap()
                        )
                );
            } else {
                saw_failure = true;
                let err = contract::execute(
                    deps.as_mut(),
                    env.clone(),
                    mock_info(owner, &[]),
                    ExecuteMsg::CompleteObjective {
                        objective_id: objective.objective_id,
                        xyz_id: xyz_id.to_string(),
                    },
                )
                .unwrap_err();
                assert_eq!(err, ContractError::ObjectiveNotStarted {});
            }
        }
        assert!(saw_success);
        assert!(saw_failure);
    }
}

#[test]
fn test_prize_pool_deposit() {
    let mut deps = mock_dependencies(&[]);
    setup_contract(deps.as_mut());

    // can't deposit if not owner
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(NONOWNER, &[Coin::new(123, "uusd")]),
        ExecuteMsg::PrizePoolDeposit {},
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // deposit must include uusd
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[Coin::new(123, "uluna")]),
        ExecuteMsg::PrizePoolDeposit {},
    )
    .unwrap_err();
    assert_eq!(err, ContractError::FailedToDepositPrizePool {});

    // valid initial deposit
    let _ = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[Coin::new(123, "uusd")]),
        ExecuteMsg::PrizePoolDeposit {},
    )
    .unwrap();
    let completed = Completed::default();
    assert_eq!(
        completed
            .get_deposited_reward_funds(deps.as_ref().storage)
            .unwrap(),
        Coin::new(123, "uusd")
    );

    // second deposit gets added to existing balance
    let _ = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[Coin::new(123, "uusd")]),
        ExecuteMsg::PrizePoolDeposit {},
    )
    .unwrap();
    assert_eq!(
        completed
            .get_deposited_reward_funds(deps.as_ref().storage)
            .unwrap(),
        Coin::new(246, "uusd")
    );

    // enable quest completions
    let _ = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        ExecuteMsg::AllowQuestClaims { allow_claims: true },
    )
    .unwrap();

    // can't deposit if quest completions are enabled
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[Coin::new(123, "uusd")]),
        ExecuteMsg::PrizePoolDeposit {},
    )
    .unwrap_err();
    assert_eq!(err, ContractError::FailedToDepositPrizePool {});
}

#[test]
fn test_complete_quest() {
    let xyz_balances = initial_xyz_balances();
    let mut deps = mock_dependencies_custom(xyz_balances, &[]);
    setup_contract(deps.as_mut());

    // deposit some reward balance
    let _ = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[Coin::new(257, "uusd")]),
        ExecuteMsg::PrizePoolDeposit {},
    )
    .unwrap();

    // can't complete quest if completions aren't enabled
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        ExecuteMsg::CompleteQuest {
            xyz_id: "xyz #1".to_string(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::TemporarilyUnableToCompleteQuest {});

    // enable quest completions
    let _ = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        ExecuteMsg::AllowQuestClaims { allow_claims: true },
    )
    .unwrap();

    // can't complete quest for xyz you don't own
    let err = contract::execute(
        deps.as_mut(),
        mock_env(),
        mock_info(OWNER, &[]),
        ExecuteMsg::CompleteQuest {
            xyz_id: "xyz #2".to_string(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // can't complete quest if not all objectives completed
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(MOCK_QUEST_TIMESTAMP + FIVE_DAYS + 1);
    let err = contract::execute(
        deps.as_mut(),
        env.clone(),
        mock_info(OWNER, &[]),
        ExecuteMsg::CompleteQuest {
            xyz_id: "xyz #1".to_string(),
        },
    )
    .unwrap_err();
    assert_eq!(err, ContractError::UnableToCompleteQuest {});

    // complete all objectives for both xyzs
    env.block.time = env.block.time.minus_seconds(2);
    let objectives = from_binary::<Vec<Objective>>(
        &contract::query(deps.as_ref(), env.clone(), QueryMsg::GetObjectives {}).unwrap(),
    )
    .unwrap();
    for (owner, xyz_id) in vec![(OWNER, "xyz #1"), (NONOWNER, "xyz #2")] {
        for objective in objectives.iter() {
            let _ = contract::execute(
                deps.as_mut(),
                env.clone(),
                mock_info(owner, &[]),
                ExecuteMsg::CompleteObjective {
                    objective_id: objective.objective_id,
                    xyz_id: xyz_id.to_string(),
                },
            )
            .unwrap();
        }
    }

    env.block.time = env.block.time.plus_seconds(2);
    for (owner, xyz_id) in vec![(OWNER, "xyz #1"), (NONOWNER, "xyz #2")] {
        // can claim half the prize pool
        let res = contract::execute(
            deps.as_mut(),
            env.clone(),
            mock_info(owner, &[]),
            ExecuteMsg::CompleteQuest {
                xyz_id: xyz_id.to_string(),
            },
        )
        .unwrap();
        assert_eq!(
            res,
            Response::new()
                .add_attribute("method", "execute")
                .add_attribute("action", "complete_quest")
                .add_attribute("reward", "{\"denom\":\"uusd\",\"amount\":\"128\"}")
                .add_message(BankMsg::Send {
                    to_address: owner.to_string(),
                    amount: vec![Coin::new(128, "uusd")]
                })
        );

        // can't claim twice
        let err = contract::execute(
            deps.as_mut(),
            env.clone(),
            mock_info(owner, &[]),
            ExecuteMsg::CompleteQuest {
                xyz_id: xyz_id.to_string(),
            },
        )
        .unwrap_err();
        assert_eq!(err, ContractError::UnableToCompleteQuestAgain {});
    }
}

#[test]
fn test_query_completed() {
    let xyz_balances = initial_xyz_balances();
    let mut deps = mock_dependencies_custom(xyz_balances, &[]);
    setup_contract(deps.as_mut());

    let xyz_id = "xyz #1".to_string();

    // Returns an empty list when none have been completed for the given xyz
    let completes = from_binary::<Vec<CompleteObjective>>(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetCompleted {
                xyz_id: xyz_id.clone(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(completes.len(), 0);

    // Complete two objectives
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(MOCK_QUEST_TIMESTAMP).plus_seconds(ONE_DAY * 2 + 1);
    for objective_id in 0..2 {
        contract::execute(
            deps.as_mut(),
            env.clone(),
            mock_info(OWNER, &[]),
            ExecuteMsg::CompleteObjective {
                objective_id,
                xyz_id: xyz_id.to_string(),
            },
        )
        .unwrap();
    }

    // Check that the completions now show up in the results
    let completes = from_binary::<Vec<CompleteObjective>>(
        &contract::query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetCompleted {
                xyz_id: xyz_id.clone(),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        completes
            .iter()
            .map(|c| c.objective.objective_id)
            .collect::<Vec<u32>>(),
        vec![0, 1]
    );
}

#[test]
fn test_query_objectives() {
    let xyz_balances = initial_xyz_balances();
    let mut deps = mock_dependencies_custom(xyz_balances, &[]);
    setup_contract(deps.as_mut());

    // Returns objectives without goals for early block time
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(0);
    let objectives = from_binary::<Vec<Objective>>(
        &contract::query(deps.as_ref(), env.clone(), QueryMsg::GetObjectives {}).unwrap(),
    )
    .unwrap();
    assert!(objectives.iter().all(|o| o.goal.is_none()));

    // Returns objectives without goals for no possible goals
    let quest_start_time = Timestamp::from_seconds(MOCK_QUEST_TIMESTAMP.clone());
    let _objectives = vec![
        Objective {
            objective_id: 0,
            objective_start_time: quest_start_time.plus_seconds(ONE_DAY),
            duration: ONE_DAY,
            multiplier: 1000,
            goal: None,
            late_penalty: 0,
            possible_goals_info: vec![],
            desc: None,
        },
        Objective {
            objective_id: 1,
            objective_start_time: quest_start_time.plus_seconds(ONE_DAY * 2),
            duration: ONE_DAY,
            multiplier: 5000,
            goal: None,
            late_penalty: 0,
            possible_goals_info: vec![],
            desc: None,
        },
        Objective {
            objective_id: 2,
            objective_start_time: quest_start_time.plus_seconds(ONE_DAY * 3),
            duration: ONE_DAY,
            multiplier: 10000,
            goal: None,
            late_penalty: 0,
            possible_goals_info: vec![],
            desc: None,
        },
    ];
    let msg = InstantiateMsg {
        config: mock_config(None, Some(_objectives)),
    };
    let _res = contract::instantiate(deps.as_mut(), mock_env(), mock_info(OWNER, &[]), msg).unwrap();
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(0);
    let objectives = from_binary::<Vec<Objective>>(
        &contract::query(deps.as_ref(), env.clone(), QueryMsg::GetObjectives {}).unwrap(),
    )
    .unwrap();
    assert!(objectives.iter().all(|o| o.goal.is_none()));

    setup_contract(deps.as_mut());
    // Returns some objectives with goals and some without for block time
    // partway through the quest.
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(MOCK_QUEST_TIMESTAMP).plus_seconds(ONE_DAY * 2 + 1);
    let objectives = from_binary::<Vec<Objective>>(
        &contract::query(deps.as_ref(), env.clone(), QueryMsg::GetObjectives {}).unwrap(),
    )
    .unwrap();
    assert!(objectives[0].goal.is_some());
    assert!(objectives[1].goal.is_some());
    assert!(objectives[2].goal.is_none());

    // Returns expected goals for given objectives (also, doesn't care about quest expiry)
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(MOCK_QUEST_TIMESTAMP).plus_seconds(FIVE_DAYS * 10);
    let objectives = from_binary::<Vec<Objective>>(
        &contract::query(deps.as_ref(), env.clone(), QueryMsg::GetObjectives {}).unwrap(),
    )
    .unwrap();
    assert_eq!(
        objectives
            .iter()
            .map(|o| o.goal.clone().unwrap())
            .collect::<Vec<Goal>>(),
        vec![
            Goal {
                name: "goal1".to_string(),
                xp_reward: Uint128::new(1),
                required_resources: vec![
                    RequiredResource {
                        resource_id: "xyzROCK".to_string(),
                        required_amount: Uint128::new(500)
                    },
                    RequiredResource {
                        resource_id: "xyzMETAL".to_string(),
                        required_amount: Uint128::new(500)
                    }
                ]
            },
            Goal {
                name: "goal3".to_string(),
                xp_reward: Uint128::new(1),
                required_resources: vec![
                    RequiredResource {
                        resource_id: "xyzROCK".to_string(),
                        required_amount: Uint128::new(4000)
                    },
                    RequiredResource {
                        resource_id: "xyzWATER".to_string(),
                        required_amount: Uint128::new(750)
                    },
                    RequiredResource {
                        resource_id: "xyzLIFE".to_string(),
                        required_amount: Uint128::new(250)
                    },
                ]
            },
            Goal {
                name: "goal1".to_string(),
                xp_reward: Uint128::new(1),
                required_resources: vec![
                    RequiredResource {
                        resource_id: "xyzROCK".to_string(),
                        required_amount: Uint128::new(5000)
                    },
                    RequiredResource {
                        resource_id: "xyzMETAL".to_string(),
                        required_amount: Uint128::new(5000)
                    }
                ]
            }
        ]
    );
}

#[test]
fn test_update_objectives() {
    let xyz_balances = initial_xyz_balances();
    let mut deps = mock_dependencies_custom(xyz_balances, &[]);
    setup_contract(deps.as_mut());

    let quest_start_time = Timestamp::from_seconds(MOCK_QUEST_TIMESTAMP.clone());
    let objectives = vec![
        Objective {
            objective_id: 0,
            objective_start_time: quest_start_time.plus_seconds(ONE_DAY),
            duration: ONE_DAY,
            multiplier: 1000,
            goal: None,
            late_penalty: 0,
            possible_goals_info: vec![],
            desc: None,
        },
        Objective {
            objective_id: 1,
            objective_start_time: quest_start_time.plus_seconds(ONE_DAY * 2),
            duration: ONE_DAY,
            multiplier: 5000,
            goal: None,
            late_penalty: 0,
            possible_goals_info: vec![],
            desc: None,
        },
        Objective {
            objective_id: 2,
            objective_start_time: quest_start_time.plus_seconds(ONE_DAY * 3),
            duration: ONE_DAY,
            multiplier: 10000,
            goal: None,
            late_penalty: 0,
            possible_goals_info: vec![],
            desc: None,
        },
    ];

    let msg = InstantiateMsg {
        config: mock_config(None, Some(objectives)),
    };
    let _res = contract::instantiate(deps.as_mut(), mock_env(), mock_info(OWNER, &[]), msg).unwrap();

    // Updates the config with Objective w/ possible_goals_info
    let possible_goals_info = vec![
        GoalInfo {
            name: "goal1".to_string(),
            xp_amount: Uint128::new(1),
            rock_weighting: Some(Uint64::new(50)),
            ice_weighting: None,
            metal_weighting: Some(Uint64::new(50)),
            gas_weighting: None,
            water_weighting: None,
            gem_weighting: None,
            life_weighting: None,
        },
        GoalInfo {
            name: "goal2".to_string(),
            xp_amount: Uint128::new(1),
            rock_weighting: Some(Uint64::new(80)),
            ice_weighting: None,
            metal_weighting: None,
            gas_weighting: Some(Uint64::new(20)),
            water_weighting: None,
            gem_weighting: None,
            life_weighting: None,
        },
        GoalInfo {
            name: "goal3".to_string(),
            xp_amount: Uint128::new(1),
            rock_weighting: Some(Uint64::new(80)),
            ice_weighting: None,
            metal_weighting: None,
            gas_weighting: None,
            water_weighting: Some(Uint64::new(15)),
            gem_weighting: None,
            life_weighting: Some(Uint64::new(5)),
        },
    ];
    let msg = ExecuteMsg::UpdateObjective {
        objective_id: 1,
        possible_goal_info: possible_goals_info.clone()
    };
    let res = contract::execute(deps.as_mut(), mock_env(), mock_info(OWNER, &[]), msg);
    assert_eq!(
        res.unwrap(),
        Response::new()
            .add_attribute("method", "execute")
            .add_attribute("action", "update_objective")
    );

    let config = CONFIG.load(deps.as_mut().storage);
    let _config = config.unwrap();
    assert_eq!(_config.clone().objectives[1].possible_goals_info, possible_goals_info);
    assert!(_config.clone().objectives[0].possible_goals_info.is_empty());
    assert!(_config.clone().objectives[2].possible_goals_info.is_empty());
}

#[test]
fn test_query_objective_count() {
    let xyz_balances = initial_xyz_balances();
    let mut deps = mock_dependencies_custom(xyz_balances, &[]);
    setup_contract(deps.as_mut());

    // Returns objectives without goals for early block time
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(MOCK_QUEST_TIMESTAMP).plus_seconds(ONE_DAY * 2 + 1);
    let _objectives = from_binary::<Vec<Objective>>(
        &contract::query(deps.as_ref(), env.clone(), QueryMsg::GetObjectives {}).unwrap(),
    )
    .unwrap();

    let count = from_binary::<Uint128>(
        &contract::query(deps.as_ref(), env.clone(), QueryMsg::GetObjectiveCompletedCount { objective_id: 0u32 }).unwrap()
    ).unwrap();
    assert_eq!(count, 0u128.into());

    let _res = contract::execute(
        deps.as_mut(),
        env.clone(),
        mock_info(OWNER, &[]),
        ExecuteMsg::CompleteObjective {
            objective_id: 0,
            xyz_id: "xyz #1".to_string(),
        },
    );
    assert!(_res.is_ok());

    let count = from_binary::<Uint128>(
        &contract::query(deps.as_ref(), env.clone(), QueryMsg::GetObjectiveCompletedCount { objective_id: 0 }).unwrap()
    ).unwrap();
    assert_eq!(count, 1u128.into());

    let _res = contract::execute(
        deps.as_mut(),
        env.clone(),
        mock_info(NONOWNER, &[]),
        ExecuteMsg::CompleteObjective {
            objective_id: 0,
            xyz_id: "xyz #2".to_string(),
        },
    );

    let count = from_binary::<Uint128>(
        &contract::query(deps.as_ref(), env.clone(), QueryMsg::GetObjectiveCompletedCount { objective_id: 0 }).unwrap()
    ).unwrap();
    assert_eq!(count, 2u128.into());
}