use collectxyz_planet_metaverse::experience::XyzExperienceMintInfo;
use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{Config, ResourceGatherInfo};

/// Information required to generate coordinate metadata which represents planets.
/// 
/// * `config` - The [Config] for this contract
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub config: Config,
    pub resource_gathering_info: Vec<ResourceGatherInfo>,
}

impl InstantiateMsg {
    pub fn is_valid(resource_gathering_info: &Vec<ResourceGatherInfo>) -> bool {
        for resource in resource_gathering_info {
            if resource.max_deviation_yield > 100 {
                return false;
            }
        }
        return true;
    }
}

/// All Fields are uptional similar to HTTP PATCH
/// 
/// * `resource_identifier` - The id for the resource. This is unique across all resources
/// * `resource_contract_address` - Contract address for the Resource
/// * `max_deviation_yield` - maximum deviation used to adjust gathered resources.
/// * `gather_task_duration_seconds` - seconds requied from start time to gather resources
/// * `gather_task_expiration_seconds` - seconds from start time before the task expires and can no longer be claimed.
/// * `randomness_contract_address`
/// * `planet_contract_address`
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct UpdateConfigData {
    pub planet_contract_address: Option<Addr>,
    pub randomness_contract_address: Option<Addr>,
    pub xyz_nft_contract_address: Option<Addr>,
    pub resource_gathering_info: Option<Vec<ResourceGatherInfo>>,
    pub gather_task_duration_seconds: Option<u64>,
    pub gather_task_expiration_seconds: Option<u64>,
    pub bonus_token_probability: Option<u8>,
    pub start_task_fee: Option<Coin>,
    pub experience_mint_config: Option<XyzExperienceMintInfo>,
}

/// This is the current migration message. 
/// This will change across contract versions if more data is required to migrate a contract
/// 
/// For now this is the same as [InstantiateMsg]
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    StartTask {
        xyz_nft_id: String,
    },

    CompleteTask {
        xyz_nft_id: String,
    },

    UpdateConfig {
        update_data: UpdateConfigData
    },
    
    Withdraw {
        amount: Vec<Coin>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCurrentConfig {},
    GetTaskForNft {
        xyz_nft_id: String,
    }
}