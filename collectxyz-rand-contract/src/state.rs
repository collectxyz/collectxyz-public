use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

use cosmwasm_std::{
    to_vec, Addr, Binary, ContractResult, Empty, QuerierWrapper, QueryRequest, StdError, StdResult,
    Storage, SystemResult, Timestamp, WasmQuery,
};
use cw_storage_plus::{Item, Map, U64Key};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Seed {
    pub contract_addr: Addr,
    pub query: String,
}

impl Seed {
    pub fn query(&self, querier: QuerierWrapper) -> StdResult<Vec<u8>> {
        let result = querier.raw_query(&to_vec(&QueryRequest::<Empty>::Wasm(WasmQuery::Smart {
            contract_addr: self.contract_addr.to_string(),
            msg: Binary(self.query.as_bytes().to_vec()),
        }))?);

        match result {
            SystemResult::Ok(ContractResult::Ok(value)) => Ok(value.to_vec()),
            _ => Err(StdError::generic_err("uh oh")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TimeSlot {
    pub slot: u64,
}

impl TimeSlot {
    pub fn new(slot: u64) -> Self {
        Self { slot }
    }

    pub fn from_timestamp(ts: Timestamp, slot_size: u64) -> Self {
        let ts_nanos = ts.nanos();
        let slot = ts_nanos - (ts_nanos % slot_size);
        Self { slot }
    }

    pub fn from_slot_size_config(storage: &dyn Storage, ts: Timestamp) -> StdResult<TimeSlot> {
        let config = CONFIG.load(storage)?;
        Ok(TimeSlot::from_timestamp(ts, config.time_slot_nanos))
    }

    pub fn from_bytes_unsafe(bytes: &[u8]) -> Self {
        let slot = u64::from_be_bytes(bytes.try_into().unwrap());
        Self { slot }
    }

    pub fn into_key(&self) -> U64Key {
        U64Key::from(self.slot)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub seeds: Vec<Seed>,
    pub time_slot_nanos: u64,
    /// NOTE: this config arg does nothing. Entries do not expire.
    pub expiry_nanos: u64,
    pub cw20_contract: Addr,
    pub minting_addresses: Vec<Addr>,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const RAND: Map<U64Key, Vec<u8>> = Map::new("rand");
pub const OWNER: Item<Addr> = Item::new("owner");
