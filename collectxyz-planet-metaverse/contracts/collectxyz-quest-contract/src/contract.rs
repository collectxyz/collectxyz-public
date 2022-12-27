use crate::execute as ExecHandler;
use crate::query as QueryHandler;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use serde::Serialize;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:collectxyz-quest-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    ExecHandler::execute_instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CompleteObjective {
            xyz_id,
            objective_id,
        } => ExecHandler::execute_complete_objective(deps, env, info, xyz_id, objective_id),
        ExecuteMsg::CompleteQuest {
            xyz_id
        } => ExecHandler::execute_complete_quest(deps, env, info, xyz_id),
        ExecuteMsg::PrizePoolDeposit {} => ExecHandler::execute_prize_pool_deposit(deps, info),
        ExecuteMsg::AllowQuestClaims { allow_claims } => ExecHandler::execute_set_allow_quest_completion(deps, info, allow_claims),
        ExecuteMsg::UpdateObjective { objective_id, possible_goal_info } => ExecHandler::execute_update_objective(deps, info, env.block, objective_id, possible_goal_info)
    }
}

fn as_binary<T>(data: &T) -> Result<Binary, ContractError>
where
    T: Serialize + ?Sized,
{
    let binary = to_binary(data)?;
    Ok(binary)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::GetCompleted { xyz_id } => {
            as_binary(&QueryHandler::query_completed(xyz_id, deps.storage)?)
        }
        QueryMsg::CurrentConfig {} => as_binary(&QueryHandler::query_config(deps.storage)?),
        QueryMsg::GetObjectives {} => as_binary(&QueryHandler::query_objectives(
            deps.storage,
            &deps.querier,
            &env.block,
        )?),
        QueryMsg::GetIsQuestCompleted {xyz_id} => as_binary(&QueryHandler::query_is_quest_completed(xyz_id, &deps)?),
        QueryMsg::GetReward { xyz_id } => as_binary(&QueryHandler::query_reward(xyz_id, &env, &deps)?),
        QueryMsg::GetObjectiveCompletedCount { objective_id } => as_binary(&QueryHandler::query_completed_count(objective_id, &deps)?)
    }
}
