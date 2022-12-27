#![cfg(not(target_arch = "wasm32"))]
use crate::msg::ResourceGenerationInfo;
use collectxyz::nft::{ XyzExtension, XyzTokenInfo};
use cosmwasm_std::{Addr, Timestamp};
use cw2::ContractVersion;

use crate::msg::RichnessThreshold;
use collectxyz_planet_metaverse::discover_planets::{Planet, PlanetResource, PlanetCoordinates};
use collectxyz_planet_metaverse::tasks::Task;

pub const EIGHTY_PERCENT: u8 = 204;
pub const THIRTY_PERCENT: u8 = 77;
pub const NOW: u64 = 1632196219;

pub const TWO_DAYS: u64 = 60 * 60 * 24 * 2;
pub const MAX_ALLOWED_PLANETS: u8 = 10;

pub const RANDOM_ADDRESS: &str = "random";
pub const NFT_OWNER_ADDRESS: &str = "nft_owner_address";
pub const NFT_OWNER_ADDRESS_2: &str = "nft_owner_address_2";
pub const XYZ_NFT_CONTRACT_ADDRESS: &str = "nft_address";
pub const XYZ_NFT_ID: &str = "xyz #1";
pub const XYZ_NFT_ID_2: &str = "xyz #2";

pub const DEFAULT_DISCOVERY_EXPIRATION_WINDOW: u64 = 3600*24*14;

pub const DEFAULT_RESOURCE_IDENTIFIER: &str = "XYZiron";
pub const DEFAULT_RESOURCE_CONTRACT_ADDR: &str = "resourceContractAddr";

pub const DEFAULT_MAX_BONUS_TOKEN_COUNT: u8 = 1;
pub const DEFAULT_BOOST_PER_BONUS_TOKEN: u8 = 5;
pub const DEFAULT_CW20_BONUS_TOKEN_CONTRACT: &str = "cw20_bonus_token";

pub const DEFAULT_RAND: &[u8] = &[
    74, 105, 74, 67, 234, 167, 221, 41, 135, 253, 217, 0, 27, 157, 136, 30, 188, 154, 145, 200,
    106, 45, 14, 41, 182, 246, 201, 59, 222, 161, 72, 59, 52, 53, 221, 151, 153, 107, 172, 42, 148,
    39, 39, 120, 248, 173, 15, 23, 206, 115, 151, 176, 154, 204, 23, 66, 161, 157, 77, 220, 192,
    31, 15, 144,
];

// ---------------------------------------------------------------- Helpers

pub fn default_richness_threshold() -> RichnessThreshold {
    return RichnessThreshold {
        level_one: 178,
        level_two: 230,
        level_three: 243,
        level_four: 251,
        level_five: 255,
    };
}

pub fn default_resource_generation_info(addr: &str) -> ResourceGenerationInfo {
    return ResourceGenerationInfo {
        resource_identifier: DEFAULT_RESOURCE_IDENTIFIER.to_string(),
        resource_contract_address: addr.to_string(),
        appearance_probability: THIRTY_PERCENT,
        richness_thresholds: default_richness_threshold(),
    };
}

pub fn default_resource_generation_info_with_id(id: &str) -> ResourceGenerationInfo {
    return ResourceGenerationInfo {
        resource_identifier: id.to_string(),
        resource_contract_address: DEFAULT_RESOURCE_CONTRACT_ADDR.to_string(),
        appearance_probability: THIRTY_PERCENT,
        richness_thresholds: default_richness_threshold(),
    };
}

pub fn default_contract_version() -> ContractVersion {
    return ContractVersion {
        contract: String::from("contract"),
        version: String::from("v1.1"),
    };
}

pub fn default_task() -> Task {
    return Task::new(
        &XYZ_NFT_ID.to_string(), 
        &Timestamp::from_seconds(NOW), 
        DEFAULT_BOOST_PER_BONUS_TOKEN,
        &default_xyz_coords_1(),
        TWO_DAYS, 
        DEFAULT_DISCOVERY_EXPIRATION_WINDOW
    );
    
}

pub fn default_resource() -> PlanetResource {
    return PlanetResource {
        resource_identifier: DEFAULT_RESOURCE_IDENTIFIER.to_string(),
        resource_richness_score: 1,
    };
}

pub fn default_planet(xyz_nft_id: &String, id: Option<String>) -> Planet {
    return Planet {
        discovered_by: String::from(xyz_nft_id),
        planet_id: id,
        resources: vec![default_resource()],
        discovery_time: Timestamp::from_seconds(1633193676),
        discovered_contract_version: Some(default_contract_version()),
        coordinates: PlanetCoordinates::from(default_xyz_coords_1())
    };
}

pub fn default_planet_by_coord(coordinates: &PlanetCoordinates, id: Option<String>) -> Planet {
    return Planet {
        discovered_by: XYZ_NFT_ID.to_string(),
        planet_id: id,
        resources: vec![default_resource()],
        discovery_time: Timestamp::from_seconds(1633193676),
        discovered_contract_version: Some(default_contract_version()),
        coordinates: coordinates.clone()
    };
}

pub fn default_xyz_coords_1() -> PlanetCoordinates {
    return PlanetCoordinates {
        x: 100,
        y: 100,
        z: 100
    };
}

pub fn default_xyz_coords_2() -> PlanetCoordinates {
    return PlanetCoordinates {
        x: 200,
        y: 200,
        z: 200
    };
}

pub fn default_xyz_nft_data(now_seconds: u64, has_arrived: bool, coordinates: Option<PlanetCoordinates>) -> XyzTokenInfo {
    return XyzTokenInfo {
        owner: Addr::unchecked(NFT_OWNER_ADDRESS.to_string()),
        approvals: vec![],
        description: "".to_string(),
        image: None,
        name: "".to_string(),
        extension: default_xyz_extension(now_seconds, has_arrived, coordinates)
    }
}

pub fn default_xyz_extension(now_seconds: u64, has_arrived: bool, coordinates: Option<PlanetCoordinates>) -> XyzExtension {
    return XyzExtension {
        coordinates: PlanetCoordinates::to_xyz_coordinates(&coordinates.unwrap_or(default_xyz_coords_1())).unwrap(),
        prev_coordinates: None,
        arrival: Timestamp::from_seconds(if has_arrived { now_seconds - 1 } else { now_seconds + 1 })
    }
}