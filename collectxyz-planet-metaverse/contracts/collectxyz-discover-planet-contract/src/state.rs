use collectxyz_planet_metaverse::{experience::XyzExperienceMintInfo, tasks::TaskRepository};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, StdResult, Storage};
use cw_storage_plus::{Item, Map};

use crate::msg::{ResourceGenerationInfo};

pub const OLD_CONFIG: Item<OldConfig> = Item::new("config");
pub const CONFIG: Item<Config> = Item::new("config_v2");
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const TASK_REPOSITORY: TaskRepository = TaskRepository {
    idx_namespace: "planet_discovery_identifier",
    pk_namespace: "planet_discovery"
};

/// Lookup table for Resource Contract Address lookup
const RESOURCE_ADDRESS_LOOKUP: Map<&str, String> = Map::new("resource_address_lookup");

/// Temporary Old Config for migration
/// TODO remove
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct OldConfig {
    pub probability_of_discovery: u8,
    pub required_seconds: u64,
    pub resource_generation_info: Vec<ResourceGenerationInfo>,
    pub core_resource_generation_info: Vec<ResourceGenerationInfo>,
    pub maximum_planets_per_coord: u8,
    pub randomness_contract_address: Addr,
    pub xyz_nft_contract_address: Addr,
    pub discovery_task_expiration_window_seconds: u64,
    pub max_number_of_bonus_tokens: u8,
    pub boost_per_bonus_token: u8,
    pub cw20_bonus_token_contract: Addr,
}

/// # Config to store the data required for task managemnt
/// 
/// * `probability_of_discovery` - Probability of discovering a planet upon task completion (0-255)
/// * `required_seconds` - Time required to complete a task (in seconds)
/// * `resource_generation_info` - `ResourceGenerationInfo` for less common resources
/// * `core_resource_generation_info` - `ResourceGenerationInfo` for core resources.
///     At least one of these will be granted if a planet is discovered
/// * `randomness_contract_address` - Contract which will be used for
///     randomness when generating planets
/// * `xyz_nft_contract_address` - XYZ NFT contract addres used for various verifications
/// * `discovery_task_expiration_window_seconds` - Seconds from task start time until discovery task expires.
/// * `boost_per_bonus_token` - how much probability boost is granted per bonus token used at Task start time
/// * `cw20_bonus_token_contract` - token address which is called to burn the tokens being spent.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub probability_of_discovery: u8,
    pub required_seconds: u64,
    pub resource_generation_info: Vec<ResourceGenerationInfo>,
    pub core_resource_generation_info: Vec<ResourceGenerationInfo>,
    pub maximum_planets_per_coord: u8,
    pub randomness_contract_address: Addr,
    pub xyz_nft_contract_address: Addr,
    pub discovery_task_expiration_window_seconds: u64,
    pub max_number_of_bonus_tokens: u8,
    pub boost_per_bonus_token: u8,
    pub cw20_bonus_token_contract: Addr,
    pub start_task_fee: Coin,
    pub experience_mint_config: XyzExperienceMintInfo,
}

pub fn save(
    storage: &mut dyn Storage,
    resource_identifier: &str,
    contract_address: &String,
) -> StdResult<()> {
    return RESOURCE_ADDRESS_LOOKUP.save(
        storage,
        resource_identifier,
        &contract_address.to_string(),
    );
}
