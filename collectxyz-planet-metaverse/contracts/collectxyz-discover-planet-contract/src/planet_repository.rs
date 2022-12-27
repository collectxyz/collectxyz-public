use std::iter::FromIterator;

use cosmwasm_std::{BlockInfo, Order, StdResult, Storage};
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, Item, MultiIndex, index_string};

use collectxyz_planet_metaverse::discover_planets::{Planet, PlanetCoordinates};

pub const TOTOAL_CLAIMED_PLANETS: Item<u64> = Item::new("total_claimed_planets");

pub struct ClaimedPlanetIndexes<'a> {
    pub coordinates: MultiIndex<'a, (Vec<u8>, Vec<u8>), Planet>,
    pub discovered_by: MultiIndex<'a, (Vec<u8>, Vec<u8>), Planet>
}

impl<'a> IndexList<Planet> for ClaimedPlanetIndexes<'a> {
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn Index<Planet>> + '_> {
        let v: Vec<&dyn Index<Planet>> = vec![&self.coordinates];
        Box::new(v.into_iter())
    }
}

const CLAIMED_PLANETS_NAMESPACE: &str = "claimed_planets";

pub fn claimed_planets_repository<'a>(
) -> IndexedMap<'a, &'a [u8], Planet, ClaimedPlanetIndexes<'a>> {
    let indexes = ClaimedPlanetIndexes {
        coordinates: MultiIndex::new(
            |d: &Planet, k: Vec<u8>| (d.coordinates.to_bytes(), k),
            CLAIMED_PLANETS_NAMESPACE,
            "claimed_planets__coord",
        ),
        discovered_by: MultiIndex::new(
            |d: &Planet, k: Vec<u8>| (index_string(&d.discovered_by), k),
            CLAIMED_PLANETS_NAMESPACE,
            "claimed_planets__discovered_by",
        ),
    };
    IndexedMap::new(CLAIMED_PLANETS_NAMESPACE, indexes)
}

pub fn count_planets_for_coordinate(storage: &dyn Storage, coordinates: &PlanetCoordinates) -> usize {
    let count: usize = claimed_planets_repository()
        .idx
        .coordinates
        .prefix(coordinates.to_bytes())
        .range(storage, None, None, Order::Ascending)
        .count();
    return count;
}

pub fn save_planet(planet: &Planet, store: &mut dyn Storage, block_info: &BlockInfo) -> StdResult<Planet> {
    let new_claimed_planets_count = TOTOAL_CLAIMED_PLANETS.load(store).unwrap_or(0) + 1;
    let id = String::from_iter([new_claimed_planets_count.to_string(), "_".to_string(), block_info.time.to_string()]);
    let mut planet_to_save =  planet.clone();
    planet_to_save.planet_id = Some(id.to_string());
    let _ = claimed_planets_repository().save(store, &index_string(&id), &planet_to_save)?;
    TOTOAL_CLAIMED_PLANETS.save(store, &new_claimed_planets_count)?;
    return Ok(planet_to_save);
}

const MAX_LIMIT: u32 = 20u32;

pub fn fetch_all_planets_for_coordinate(
    storage: &dyn Storage,
    coordinates: &PlanetCoordinates,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<Planet>> {
    let limit = limit.unwrap_or(MAX_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    return claimed_planets_repository()
        .idx
        .coordinates
        .prefix(coordinates.to_bytes())
        .range(storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(_, planet)| planet as Planet))
        .collect();
}
