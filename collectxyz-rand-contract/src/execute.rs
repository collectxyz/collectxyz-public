use sha2::{Digest, Sha512};

use cosmwasm_std::{
    to_binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, MigrateMsg};
use crate::state::{Config, TimeSlot, CONFIG, OWNER, RAND};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    OWNER.save(deps.storage, &info.sender)?;

    let config = Config {
        seeds: msg.seeds,
        time_slot_nanos: msg.time_slot_nanos,
        expiry_nanos: msg.expiry_nanos,
        cw20_contract: msg.cw20_contract,
        minting_addresses: msg.minting_addresses,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

pub fn execute_update_rand(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // add new randomness
    let new_time_slot = TimeSlot::from_slot_size_config(deps.storage, env.block.time)?;
    let new_rand = generate_current_rand(deps.as_ref(), env)?;
    RAND.update(deps.storage, new_time_slot.into_key(), |old| match old {
        Some(_) => Err(ContractError::BonusClaimed {}),
        None => Ok(new_rand),
    })?;

    // grant the caller a bonus token for providing randomness
    let mint_bonus_token = Cw20ExecuteMsg::Mint {
        recipient: info.sender.to_string(),
        amount: Uint128::new(1000000),
    };
    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "update_rand")
        .add_message(WasmMsg::Execute {
            contract_addr: config.cw20_contract.to_string(),
            msg: to_binary(&mint_bonus_token)?,
            funds: vec![],
        }))
}

pub fn execute_mint_bonus_token(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if !config.minting_addresses.contains(&info.sender) {
        return Err(ContractError::Unauthorized {});
    }
    let mint_bonus_token = Cw20ExecuteMsg::Mint {
        recipient: recipient,
        amount: amount,
    };
    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "mint_bonus_token")
        .add_message(WasmMsg::Execute {
            contract_addr: config.cw20_contract.to_string(),
            msg: to_binary(&mint_bonus_token)?,
            funds: vec![],
        }))
}

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    config: Config,
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    if owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "update_config"))
}

pub fn migrate(deps: DepsMut, msg: MigrateMsg) -> StdResult<Response> {
    CONFIG.remove(deps.storage);

    let config = Config {
        seeds: msg.seeds,
        time_slot_nanos: msg.time_slot_nanos,
        expiry_nanos: msg.expiry_nanos,
        cw20_contract: msg.cw20_contract,
        minting_addresses: msg.minting_addresses,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

fn generate_current_rand(deps: Deps, env: Env) -> StdResult<Vec<u8>> {
    let config = CONFIG.load(deps.storage)?;
    let seed_data: Vec<u8> = config
        .seeds
        .iter()
        .flat_map(|seed| seed.query(deps.querier).unwrap_or_default())
        .collect();

    if seed_data.is_empty() {
        return Err(StdError::generic_err("failed to pull any data!"));
    }

    let seed_data_with_ts = [seed_data, env.block.time.nanos().to_be_bytes().to_vec()].concat();

    let mut hasher = Sha512::new();
    hasher.update(seed_data_with_ts);
    let digest = hasher.finalize();

    Ok(digest.as_slice().to_vec())
}
