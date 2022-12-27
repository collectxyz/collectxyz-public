use collectxyz_planet_metaverse::experience::XyzExperienceMintInfo;
use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


/// Given a threshhold we will compare randome number from 0-255 against 
/// the threshhold values which should range from 0-255.
/// 
/// i.e. given random number 155 and level_one 125 && level_two 130 && level_three 160 
/// our richness score will be level_three since level_two < 155 < level_three
/// 
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct RichnessThreshold {
    pub level_one: u8,
    pub level_two: u8,
    pub level_three: u8,
    pub level_four: u8,
    pub level_five: u8,
}

/// Information required to generate resource metadata for planets.
/// 
/// * `resource_identifier` - The id for the resource. This is unique across all resources
/// * `resource_contract_address` - Contract address for the Resource
/// * `appearance_probability` - Probability that this resource will appear in a planet
/// * `richness_thresholds` - [RichnessThreshold] for this resource
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ResourceGenerationInfo {
    pub resource_identifier: String,
    pub resource_contract_address: String,
    pub appearance_probability: u8,
    pub richness_thresholds: RichnessThreshold
}

/// Information required to generate coordinate metadata which represents planets.
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
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
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

/// All Fields are uptional similar to HTTP PATCH
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
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct UpdateConfigData {
    pub probability_of_discovery: Option<u8>,
    pub required_seconds: Option<u64>,
    pub resource_generation_info: Option<Vec<ResourceGenerationInfo>>,
    pub core_resource_generation_info: Option<Vec<ResourceGenerationInfo>>,
    pub maximum_planets_per_coord: Option<u8>,
    pub randomness_contract_address: Option<Addr>,
    pub xyz_nft_contract_address: Option<Addr>,
    pub discovery_task_expiration_window_seconds: Option<u64>,
    pub max_number_of_bonus_tokens: Option<u8>,
    pub boost_per_bonus_token: Option<u8>,
    pub cw20_bonus_token_contract: Option<Addr>,
    pub start_task_fee: Option<Coin>,
    pub experience_mint_config: Option<XyzExperienceMintInfo>,
}

/// This is the current migration message. 
/// This will change across contract versions if more data is required to migrate a contract
/// 
/// For now this is the same as [InstantiateMsg]
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

    ///
    /// Executed to begin a task based on params from config
    /// 
    /// ### Will fail if:
    /// 1. Task has started
    /// 2. The `xyz_nft_id` is not owned by the sender
    /// 3. too many bonus tokens are being set
    /// 4. NFT is moving
    /// 5. User does not own the amount of `bonus_token_count` being burnt
    /// 6. Coords cannot discover more planets
    /// 
    StartTask {
        xyz_nft_id: String,
        bonus_token_count: u8,
    },

    ///
    /// Executed to claim task items if possible.
    /// 
    /// ### Will fail if:
    /// 1. Task has started/no Task is present
    /// 2. NFT is moving
    /// 3. NFT is not owned by the sender
    /// 4. NFT cannot discover more planets
    ///
    CompleteTask {
        xyz_nft_id: String,
    },

    /// Executed to update the planet & resource generation configs.
    UpdateConfig {
        update_config_data: UpdateConfigData
    },

    Withdraw {
        amount: Vec<Coin>,
    },
}