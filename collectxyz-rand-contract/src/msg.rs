use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp, Uint128};

use crate::state::{Config, Seed};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub seeds: Vec<Seed>,
    pub time_slot_nanos: u64,
    pub expiry_nanos: u64,
    pub cw20_contract: Addr,
    pub minting_addresses: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateRand {},
    UpdateConfig { config: Config },
    MintBonusToken { recipient: String, amount: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {
    pub seeds: Vec<Seed>,
    pub time_slot_nanos: u64,
    pub expiry_nanos: u64,
    pub cw20_contract: Addr,
    pub minting_addresses: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    LatestRand {},
    TimestampRand { timestamp: Timestamp },
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LatestRandResponse {
    pub slot: u64,
    pub rand: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TimestampRandResponse {
    pub slot: u64,
    pub rand: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub config: Config,
}
