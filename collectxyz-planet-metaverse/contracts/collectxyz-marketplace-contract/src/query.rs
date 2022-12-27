use cosmwasm_std::{Deps, Env, StdResult, Uint128};

use crate::msg::ListingsResponse;
use crate::state::{Config, Listing, Listings, CONFIG};

pub fn query_listing_info(deps: Deps, _env: Env, listing_id: u64) -> StdResult<Listing> {
    let listings = Listings::default();
    listings.fetch_listing(deps.storage, listing_id)
}

pub fn query_listings(
    deps: Deps,
    env: Env,
    lister_xyz_id: Option<String>,
    prices: Option<Vec<Uint128>>,
    resources: Option<Vec<String>>,
    include_inactive: Option<bool>,
    ascending: Option<bool>,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<ListingsResponse> {
    let listings = Listings::default();
    let result = listings.fetch_listings(
        deps.storage,
        env.block.time,
        lister_xyz_id,
        prices.unwrap_or(vec![]),
        resources.unwrap_or(vec![]),
        include_inactive.unwrap_or(false),
        ascending.unwrap_or(false),
        start_after,
        limit,
    )?;

    Ok(ListingsResponse { listings: result })
}

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}
