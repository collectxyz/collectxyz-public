use collectxyz_planet_metaverse::tasks::Task;
use cosmwasm_std::{BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Storage, entry_point, to_binary};
use cw2::set_contract_version;
use std::str;


use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, UpdateConfigData};
use crate::{complete_task, start_task};
use crate::state::{ADMIN, CONFIG, Config, ResourceGatherInfo, TASK_REPOSITORY, save_resource_gather_info};

const CONTRACT_NAME: &str = "crates.io:collectxyz-gather-resources-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if !InstantiateMsg::is_valid(&msg.resource_gathering_info) {
        return Err(StdError::generic_err("Invalid config params."))
    }

    if msg.resource_gathering_info.len() > 0 {
        update_resource_contract_lookup(
            deps.storage, 
            &msg.resource_gathering_info
        )?;
    }

    CONFIG.remove(deps.storage);
    CONFIG.save(deps.storage, &msg.config)?;
    ADMIN.save(deps.storage,&info.sender)?;
    return Ok(Response::default());
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::StartTask { xyz_nft_id } => start_task::start_task(
            &xyz_nft_id, info, &env.block, deps.storage, &deps.querier
        ),
        ExecuteMsg::CompleteTask { xyz_nft_id } => complete_task::complete_task(
            &xyz_nft_id, &info.sender.to_string(), &env.block, deps.storage, &deps.querier
        ),
        ExecuteMsg::UpdateConfig { update_data } => update_config(
            info.sender.to_string(), deps.storage, update_data
        ),
        ExecuteMsg::Withdraw { amount } => execute_withdraw(deps, env, info, amount),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCurrentConfig {  } => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::GetTaskForNft { xyz_nft_id } => to_binary(&query_existing_task(xyz_nft_id, deps.storage)),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    return Ok(Response::default());
}

pub fn execute_withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Vec<Coin>,
) -> Result<Response, StdError> {
    let admin = ADMIN.load(deps.storage)?;
    if !info.sender.eq(&admin) {
        return Err(StdError::generic_err("Only admin can execute this method."));
    }

    Ok(Response::new().add_message(BankMsg::Send {
        amount,
        to_address: admin.to_string(),
    }))
}

fn query_existing_task(xyz_nft_id: String, storage: &dyn Storage) -> Option<Task> {
    let task_result = TASK_REPOSITORY.fetch_existing_task(&xyz_nft_id, storage);
    return match task_result {
        Ok(task) => Some(task),
        Err(_) => None,
    };
}

fn update_resource_contract_lookup(
    storage: &mut dyn Storage,
    infos: &Vec<ResourceGatherInfo>,
) -> StdResult<()> {
    for resource in infos {
        save_resource_gather_info(
            storage,
            &resource
        )?;
    }
    return Ok(());
}

fn update_config(
    sender: String,
    storage: &mut dyn Storage,
    update_config_data: UpdateConfigData,
) -> Result<Response, StdError> {
    let admin = ADMIN.load(storage)?;
    if !sender.eq(&admin) {
        return Err(StdError::generic_err("Only admin can execute this method."));
    }

    if update_config_data.resource_gathering_info.is_some() {
        update_resource_contract_lookup(
            storage, 
            &update_config_data.resource_gathering_info.clone().unwrap()
        )?;
    }
    
    let current_config = CONFIG.load(storage)?;
    let new_config = Config {
        planet_contract_address: update_config_data.planet_contract_address.unwrap_or(current_config.planet_contract_address),
        randomness_contract_address: update_config_data.randomness_contract_address.unwrap_or(current_config.randomness_contract_address),
        xyz_nft_contract_address: update_config_data.xyz_nft_contract_address.unwrap_or(current_config.xyz_nft_contract_address),
        gather_task_duration_seconds: update_config_data.gather_task_duration_seconds.unwrap_or(current_config.gather_task_duration_seconds),
        gather_task_expiration_seconds: update_config_data.gather_task_expiration_seconds.unwrap_or(current_config.gather_task_expiration_seconds),
        bonus_token_probability: update_config_data.bonus_token_probability.unwrap_or(current_config.bonus_token_probability),
        start_task_fee: update_config_data.start_task_fee.unwrap_or(current_config.start_task_fee),
        experience_mint_config: update_config_data.experience_mint_config.unwrap_or(current_config.experience_mint_config.into()).into(),
    };

    CONFIG.save(storage, &new_config)?;
    return Ok(Response::default());
}

pub fn query_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}