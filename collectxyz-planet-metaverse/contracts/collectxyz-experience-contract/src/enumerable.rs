use cosmwasm_std::{Deps, Order, StdResult};
use collectxyz_experience::{AllAccountsResponse, AllAllowancesResponse, AllowanceInfo};

use crate::state::{ALLOWANCES, BALANCES};
use cw_storage_plus::Bound;

// settings for pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

pub fn query_all_allowances(
    deps: Deps,
    owner: String,
    owner_xyz_id: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<AllAllowancesResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let allowances: StdResult<Vec<AllowanceInfo>> = ALLOWANCES
        .prefix((&owner_xyz_id,&owner_addr))
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (k, v) = item?;
            Ok(AllowanceInfo {
                spender: String::from_utf8(k)?,
                allowance: v.allowance,
                expires: v.expires,
            })
        })
        .collect();
    Ok(AllAllowancesResponse {
        allowances: allowances?,
    })
}

pub fn query_all_accounts(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<AllAccountsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let accounts: Result<Vec<_>, _> = BALANCES
        .keys(deps.storage, start, None, Order::Ascending)
        .map(String::from_utf8)
        .take(limit)
        .collect();

    Ok(AllAccountsResponse {
        accounts: accounts?,
    })
}
