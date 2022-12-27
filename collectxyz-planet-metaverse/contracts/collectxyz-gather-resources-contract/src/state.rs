
use std::collections::HashMap;

use collectxyz_planet_metaverse::{discover_planets::PlanetResource, experience::XyzExperienceMintInfo, tasks::TaskRepository};
use schemars::{JsonSchema};
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, StdResult, Storage};
use cw_storage_plus::{Item, Map};

pub const OLD_CONFIG: Item<OldConfig> = Item::new("config");
pub const CONFIG: Item<Config> = Item::new("config_v2");
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const TASK_REPOSITORY: TaskRepository = TaskRepository {
    idx_namespace: "resource_gathering_identifier",
    pk_namespace: "resource_gathering"
};

/// Lookup table for Resource gather info; Global parameters encompassing all resources
pub const RESOURCE_GATHER_INFO_LOOKUP: Map<&str, ResourceGatherInfo> = Map::new("resource_gather_info_lookup");

/// Lookup table for resources being gathered for nft id
/// 
/// Set when Task starts and removed when task complete.
pub const NFT_ID_GATHERING_RESOURCES: Map<&str, Vec<PlanetResource>> = Map::new("nft_id_gathering_resources");

/// Information required gather resources.
/// 
/// * `resource_identifier` - The id for the resource. This is unique across all resources
/// * `resource_contract_address` - Contract address for the Resource
/// * `base_yield` - [u64] the constant base yield per task completion
/// * `max_deviation_yield` - maximum deviation used to adjust gathered resources.
/// * `deviation_direction_threshhold` - if random number is less than this threshhold
///     direction will be negative, otherwise positive 
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ResourceGatherInfo {
    pub resource_identifier: String,
    pub resource_contract_address: String,
    pub base_yield: u64,
    pub max_deviation_yield: u64,
    pub deviation_direction_threshhold: u8
}

/// Temporary Old Config for migration
/// TODO remove
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct OldConfig {
    pub planet_contract_address: Addr,
    pub randomness_contract_address: Addr,
    pub xyz_nft_contract_address: Addr,
    pub gather_task_duration_seconds: u64,
    pub gather_task_expiration_seconds: u64,
    pub bonus_token_probability: u8,
}

/// # Config to store the data required for task managemnt
/// 
/// * `randomness_contract_address` - Contract address for the Randomness
/// * `xyz_nft_contract_address` - NFT contract address
/// * `planet_contract_address` - Planet Contract address
/// * `gather_task_duration_seconds` - seconds requied from start time to gather resources
/// * `gather_task_expiration_seconds` - seconds from start time before the task expires and can no longer be claimed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub planet_contract_address: Addr,
    pub randomness_contract_address: Addr,
    pub xyz_nft_contract_address: Addr,
    pub gather_task_duration_seconds: u64,
    pub gather_task_expiration_seconds: u64,
    pub bonus_token_probability: u8,
    pub start_task_fee: Coin,
    pub experience_mint_config: XyzExperienceMintInfo,
}

pub fn save_resource_gather_info(
    storage: &mut dyn Storage,
    resource_gather_info: &ResourceGatherInfo,
) -> StdResult<()> {
    return RESOURCE_GATHER_INFO_LOOKUP.save(
        storage,
        &resource_gather_info.resource_identifier,
        &resource_gather_info,
    );
}

pub fn load_resource_gather_info(storage: &mut dyn Storage, resource_identifier: &str) -> StdResult<ResourceGatherInfo> {
    return RESOURCE_GATHER_INFO_LOOKUP.load(storage, resource_identifier);
}

pub fn load_all_resource_gathering_info(storage: &mut dyn Storage) -> StdResult<HashMap<String, ResourceGatherInfo>> {
    return RESOURCE_GATHER_INFO_LOOKUP.range(
        storage,
        None,
        None,
        cosmwasm_std::Order::Ascending
    ).map(|item| { 
        let (k, v) = item?;
        Ok((String::from_utf8(k)?, v))
    }).collect();
}
