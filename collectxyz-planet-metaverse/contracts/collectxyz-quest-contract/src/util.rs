use std::{
    borrow::BorrowMut,
    iter::{FromIterator},
};

use collectxyz_planet_metaverse::util::random_numbers;
use cosmwasm_std::{
    to_binary, Addr, BalanceResponse, BankQuery, Coin, Deps, Env,
    QuerierWrapper, StdResult, Uint128, WasmQuery,
};
use sha2::{Digest, Sha512};

use crate::{
    query::{query_completed, query_objectives},
    state::{Completed, Objective, CONFIG},
    ContractError,
};

const ONE_DAY: u64 = 60 * 60 * 24;

pub fn get_random_for_objective_start_time(
    querier: &QuerierWrapper,
    randomness_contract: &Addr,
    objective_id: u32,
    objective: &Objective,
) -> Result<Vec<u8>, ContractError> {
    let mut random_nums = random_numbers(
        querier,
        randomness_contract,
        objective.objective_start_time.minus_seconds(ONE_DAY),
    )?;

    let mut objective_specific_portion = String::from_iter([objective_id.to_string()])
        .as_bytes()
        .to_vec();

    random_nums.append(objective_specific_portion.borrow_mut());
    let mut hasher = Sha512::new();
    hasher.update(random_nums);
    let digest = hasher.finalize();

    return Ok(digest.as_slice().to_vec());
}

pub fn query_contract_balance(
    querier: &QuerierWrapper,
    contract_addr: &Addr,
    denom: String,
) -> StdResult<Coin> {
    let balance_msg = BankQuery::Balance {
        address: contract_addr.to_string(),
        denom,
    };

    let wasm = WasmQuery::Smart {
        contract_addr: contract_addr.to_string(),
        msg: to_binary(&balance_msg)?,
    };

    let response: BalanceResponse = querier.query(&wasm.into())?;
    return Ok(response.amount);
}

pub fn calculate_xyz_reward(
    xyz_id: &String,
    env: &Env,
    deps: &Deps,
) -> Result<Coin, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let completed = query_completed(xyz_id.clone(), deps.storage)?;
    let objectives = query_objectives(deps.storage, &deps.querier, &env.block)?;

    let completed_repo = Completed::default();

    if completed.len() != objectives.len() {
        return Err(ContractError::UnableToCompleteQuest {});
    }

    let reward_pool_funds = completed_repo.get_deposited_reward_funds(deps.storage)?;
    if reward_pool_funds.amount.is_zero() {
        return Err(ContractError::UnableToCompleteQuestNoFunds {});
    }

    let total_completion_count =
    completed_repo.total_quest_completed_count(deps.storage, &config)?;

    if total_completion_count == 0 {
        return Err(ContractError::UnableToCompleteQuest {});
    }

    let reward_amount = reward_pool_funds
        .amount
        .checked_div(Uint128::from(total_completion_count as u128))
        .unwrap_or(Uint128::zero());

    Ok(Coin::new(reward_amount.into(), "uusd".to_string()))
}