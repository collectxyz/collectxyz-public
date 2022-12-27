use collectxyz_planet_metaverse::discover_planets::PlanetCoordinates;
use cosmwasm_std::{BlockInfo, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Task {
    pub nft_token_id: String,
    pub start_time: Timestamp,
    pub expires: Timestamp,
    pub completes: Timestamp,
    pub expected_boost: u8,
    pub coordinates: PlanetCoordinates,
}

impl Task {
    pub fn new(
        nft_token_id: &String, 
        start_time: &Timestamp,
        expected_boost: u8,
        coordinates: &PlanetCoordinates,
        time_required_seconds: u64, 
        time_to_expiry_seconds: u64
    ) -> Task {

        return Task {
            nft_token_id: nft_token_id.to_string(),
            start_time: start_time.clone(),
            completes: start_time.plus_seconds(time_required_seconds),
            expires: start_time.plus_seconds(time_to_expiry_seconds),
            expected_boost: expected_boost,
            coordinates: coordinates.clone()
        };
    }

    pub fn is_task_complete(&self, block: &BlockInfo) -> bool {
        return block.time >= self.completes;
    }

    pub fn is_task_expired(&self, block: &BlockInfo) -> bool {
        return block.time >= self.expires;
    }
}