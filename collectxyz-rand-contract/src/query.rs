use cosmwasm_std::{Deps, Env, Order, StdError, StdResult, Timestamp};

use crate::msg::{ConfigResponse, LatestRandResponse, TimestampRandResponse};
use crate::state::{TimeSlot, CONFIG, RAND};

pub fn query_latest_rand(deps: Deps, _env: Env) -> StdResult<LatestRandResponse> {
    RAND.range(deps.storage, None, None, Order::Descending)
        .next()
        .map(|item| {
            item.map(|(index, rand)| LatestRandResponse {
                slot: TimeSlot::from_bytes_unsafe(&index).slot,
                rand,
            })
        })
        .unwrap_or_else(|| Err(StdError::generic_err("no rand available!")))
}

pub fn query_timestamp_rand(
    deps: Deps,
    env: Env,
    timestamp: Timestamp,
) -> StdResult<TimestampRandResponse> {
    let time_slot = TimeSlot::from_slot_size_config(deps.storage, timestamp)?;
    let rand = RAND
        .load(deps.storage, time_slot.into_key())
        .unwrap_or(query_latest_rand(deps, env)?.rand);
    Ok(TimestampRandResponse {
        rand,
        slot: time_slot.slot,
    })
}

pub fn query_config(deps: Deps, _env: Env) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { config })
}
