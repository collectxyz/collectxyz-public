#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::{get_contract_version, set_contract_version};

use crate::error::ContractError;
use crate::execute as ExecHandler;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query as QueryHandler;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:collectxyz-marketplace-contract";
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
        ExecuteMsg::MakeListing {
            lister_xyz_id,
            price_rmi,
            deposit_rmi_denom,
            resources,
        } => ExecHandler::execute_make_listing(
            deps,
            env,
            info,
            lister_xyz_id,
            price_rmi,
            deposit_rmi_denom,
            resources,
        ),
        ExecuteMsg::RevokeListing { listing_id } => {
            ExecHandler::execute_revoke_listing(deps, env, info, listing_id)
        }
        ExecuteMsg::TakeListing {
            listing_id,
            taker_xyz_id,
            rmi_denom,
        } => {
            ExecHandler::execute_take_listing(deps, env, info, listing_id, taker_xyz_id, rmi_denom)
        }
        ExecuteMsg::UpdateConfig { config_patch } => {
            ExecHandler::execute_update_config(deps, env, info, config_patch)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> StdResult<Response> {
    let version = get_contract_version(deps.storage)?;
    if version.contract != CONTRACT_NAME {
        return Err(StdError::generic_err(
            "can't migrate to contract with different name",
        ));
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ExecHandler::execute_migrate(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ListingInfo { listing_id } => {
            to_binary(&QueryHandler::query_listing_info(deps, env, listing_id)?)
        }
        QueryMsg::Listings {
            lister_xyz_id,
            prices,
            resources,
            include_inactive,
            ascending,
            start_after,
            limit,
        } => to_binary(&QueryHandler::query_listings(
            deps,
            env,
            lister_xyz_id,
            prices,
            resources,
            include_inactive,
            ascending,
            start_after,
            limit,
        )?),
        QueryMsg::Config {} => to_binary(&QueryHandler::query_config(deps, env)?),
    }
}
