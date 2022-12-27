#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError};
use cw2::{set_contract_version, get_contract_version};

use crate::error::ContractError;
use crate::execute as ExecHandler;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query as QueryHandler;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:collectxyz-rand-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    ExecHandler::instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateRand {} => ExecHandler::execute_update_rand(deps, env, info),
        ExecuteMsg::UpdateConfig { config } => {
            ExecHandler::execute_update_config(deps, env, info, config)
        },
        ExecuteMsg::MintBonusToken { recipient, amount } => ExecHandler::execute_mint_bonus_token(deps, env, info, recipient, amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::LatestRand {} => to_binary(&QueryHandler::query_latest_rand(deps, env)?),
        QueryMsg::TimestampRand { timestamp } => {
            to_binary(&QueryHandler::query_timestamp_rand(deps, env, timestamp)?)
        }
        QueryMsg::Config {} => to_binary(&QueryHandler::query_config(deps, env)?),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> StdResult<Response> {
    let version = get_contract_version(deps.storage)?;
    if version.contract != CONTRACT_NAME {
        return Err(StdError::generic_err(
            "can't migrate to contract with different name",
        ));
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ExecHandler::migrate(deps, msg)
}
