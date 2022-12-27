use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Uint128};

use crate::state::{Config, Listing, Resource};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub config: Config,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigPatch {
    pub listing_expiry_seconds: Option<u64>,
    pub listing_pending_seconds: Option<u64>,
    pub listing_deposit_percent: Option<u64>,
    pub allowed_listing_prices: Option<Vec<Uint128>>,
    pub make_listing_fee: Option<Coin>,
    pub take_listing_fee: Option<Coin>,
    pub xyz_nft_contract: Option<Addr>,
    pub rock_contract: Option<Addr>,
    pub ice_contract: Option<Addr>,
    pub metal_contract: Option<Addr>,
    pub gas_contract: Option<Addr>,
    pub water_contract: Option<Addr>,
    pub gem_contract: Option<Addr>,
    pub life_contract: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Make an listing on the marketplace. Executing this message burns the associated
    /// xyz's balance for the relevant resource to prevent spending the listed resources,
    /// plus an RMI deposit proportional to `price_rmi`. Requires that the caller owns
    /// `lister_xyz_id` and that caller provided sufficient fees (if any are configured).
    MakeListing {
        lister_xyz_id: String,
        price_rmi: Uint128,
        deposit_rmi_denom: String,
        resources: Vec<Resource>,
    },
    /// Remove an open listing from the marketplace. Executing this message mints
    /// the listed resource quantity back to the xyz that made the listing. Requires
    /// that the caller owns the xyz associated with the given listing ID.
    RevokeListing { listing_id: u64 },
    /// Take the listing using the selected RMI denom (`xyzROCK`, `xyzICE`, or `xyzMETAL`).
    /// Requires that the caller owns `taker_xyz_id` and that `taker_xyz_id` has sufficient
    /// balance of the selected RMI denom.
    TakeListing {
        listing_id: u64,
        taker_xyz_id: String,
        rmi_denom: String,
    },
    /// Make a patch update to the contract config.
    UpdateConfig { config_patch: ConfigPatch },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {
    pub config: Config,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ListingInfo {
        listing_id: u64,
    },
    Listings {
        lister_xyz_id: Option<String>,
        prices: Option<Vec<Uint128>>,
        resources: Option<Vec<String>>,
        include_inactive: Option<bool>,
        ascending: Option<bool>,
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ListingsResponse {
    pub listings: Vec<Listing>,
}
