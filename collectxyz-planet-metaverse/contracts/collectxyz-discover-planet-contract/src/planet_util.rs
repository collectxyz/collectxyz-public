use cosmwasm_std::{StdError, StdResult, Storage};

use collectxyz_planet_metaverse::discover_planets::{GetClaimedPlanetsForNftResponse, PlanetCoordinates};
use crate::planet_repository::{count_planets_for_coordinate, fetch_all_planets_for_coordinate};
use crate::state::Config;

pub fn is_planet_limit_reached(
    storage: &dyn Storage,
    config: &Config,
    coordinates: &PlanetCoordinates,
) -> bool {
    let count: usize = count_planets_for_coordinate(storage, coordinates);
    return count >= usize::from(config.maximum_planets_per_coord);
}

pub fn query_all_planets_for_coord(
    storage: &dyn Storage,
    coordinates: &PlanetCoordinates,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<GetClaimedPlanetsForNftResponse> {
    let planets = fetch_all_planets_for_coordinate(storage, coordinates, start_after, limit);
    return match planets {
        Ok(_planets) => Ok(GetClaimedPlanetsForNftResponse {
            claimed_planets: _planets,
        }),
        Err(_) => Err(StdError::generic_err("Error fetching planets")),
    };
}
