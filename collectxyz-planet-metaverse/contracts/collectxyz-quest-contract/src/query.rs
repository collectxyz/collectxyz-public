use std::convert::TryInto;

use cosmwasm_std::{BlockInfo, QuerierWrapper, Storage, Coin, Env, Deps, Uint128};

use crate::{
    state::{CompleteObjective, Completed, Config, Objective, CONFIG},
    util::{get_random_for_objective_start_time, calculate_xyz_reward},
    ContractError,
};

pub fn query_config(storage: &dyn Storage) -> Result<Config, ContractError> {
    let config = CONFIG.load(storage)?;
    return Ok(config);
}

pub fn query_objectives(
    storage: &dyn Storage,
    querier: &QuerierWrapper,
    block: &BlockInfo,
) -> Result<Vec<Objective>, ContractError> {
    let config = CONFIG.load(storage)?;

    let mut objectives_response: Vec<Objective> = vec![];
    for (id, objective) in config.objectives.iter().enumerate() {
        if objective.is_started(block) {
            // Get the objective for the objective's timestamp
            let random_nums = get_random_for_objective_start_time(
                querier,
                &config.randomness_contract,
                id.try_into().unwrap(),
                &objective,
            )?;
            let objective_with_goal = config.get_objective(block, &random_nums, &mut objective.clone())?;
            objectives_response.push(objective_with_goal);
        } else {
            objectives_response.push(objective.clone());
        }
    }

    return Ok(objectives_response);
}

pub fn query_completed(
    xyz_id: String,
    storage: &dyn Storage,
) -> Result<Vec<CompleteObjective>, ContractError> {
    Completed::default().get_completed_for(&xyz_id, storage)
}

pub fn query_reward(
    xyz_id: String,
    env: &Env,
    deps: &Deps
) -> Result<Coin, ContractError> {
    calculate_xyz_reward(&xyz_id, &env, deps)
}

pub fn query_is_quest_completed(
    xyz_id: String,
    deps: &Deps,
) -> Result<bool, ContractError> {
    return Ok(Completed::default().is_quest_already_completed_for(deps.storage, &xyz_id));
}

pub fn query_completed_count(
    objective_id: u32,
    deps: &Deps,
) -> Result<Uint128, ContractError> {
    return Ok(Completed::default().total_completed_for_objective(deps.storage, &objective_id)?);
}