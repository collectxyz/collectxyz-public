use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::iter::FromIterator;

use cosmwasm_std::{Addr, Coin, Decimal, Order, StdResult, Storage, Timestamp, Uint128};
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, Item, MultiIndex, U64Key};

use crate::error::ContractError;

pub const OWNER: Item<Addr> = Item::new("owner");
pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub listing_expiry_seconds: u64,
    pub listing_pending_seconds: u64,
    pub listing_deposit_percent: u64,
    pub allowed_listing_prices: Vec<Uint128>,
    pub make_listing_fee: Coin,
    pub take_listing_fee: Coin,
    pub xyz_nft_contract: Addr,
    pub rock_contract: Addr,
    pub ice_contract: Addr,
    pub metal_contract: Addr,
    pub gas_contract: Addr,
    pub water_contract: Addr,
    pub gem_contract: Addr,
    pub life_contract: Addr,
}

impl Config {
    pub fn resource_addr(&self, resource_id: &str) -> Result<Addr, ContractError> {
        let addr = match resource_id {
            "xyzROCK" => self.rock_contract.clone(),
            "xyzMETAL" => self.metal_contract.clone(),
            "xyzICE" => self.ice_contract.clone(),
            "xyzGAS" => self.gas_contract.clone(),
            "xyzWATER" => self.water_contract.clone(),
            "xyzGEM" => self.gem_contract.clone(),
            "xyzLIFE" => self.life_contract.clone(),
            _ => return Err(ContractError::InvalidResourceId(resource_id.to_string())),
        };

        Ok(addr)
    }

    pub fn deposit_rmi_amount(&self, price_rmi: Uint128) -> Uint128 {
        Decimal::from_ratio(self.listing_deposit_percent, 100u128) * price_rmi
    }
}

pub fn validate_resource_id(resource_id: &str) -> Result<(), ContractError> {
    let is_valid = vec![
        "xyzROCK", "xyzMETAL", "xyzICE", "xyzGAS", "xyzWATER", "xyzGEM", "xyzLIFE",
    ]
    .contains(&resource_id);

    if !is_valid {
        Err(ContractError::InvalidResourceId(resource_id.to_string()))
    } else {
        Ok(())
    }
}

pub fn validate_rmi_denom(rmi_denom: &str) -> Result<(), ContractError> {
    let is_valid = vec!["xyzROCK", "xyzMETAL", "xyzICE"].contains(&rmi_denom);

    if !is_valid {
        Err(ContractError::InvalidResourceId(rmi_denom.to_string()))
    } else {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Resource {
    pub id: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Listing {
    pub listing_id: u64,
    pub lister_xyz_id: String,
    pub price_rmi: Uint128,
    pub deposit_rmi_denom: String,
    pub deposit_rmi_amount: Uint128,
    pub created_at: Timestamp,
    pub active_at: Timestamp,
    pub expired_at: Timestamp,
    pub resources: Vec<Resource>,
}

const RESOURCE_INTEGER_AMOUNT: u128 = 1000000;

#[derive(PartialEq, Debug)]
pub enum ListingState {
    Pending,
    Active,
    Expired,
}

impl Listing {
    pub fn try_new(
        config: Config,
        listing_id: u64,
        lister_xyz_id: String,
        price_rmi: Uint128,
        deposit_rmi_denom: String,
        timestamp: Timestamp,
        resources: Vec<Resource>,
    ) -> Result<Self, ContractError> {
        // Validate the deposit RMI denom
        validate_rmi_denom(&deposit_rmi_denom)?;

        // Validate the listing price
        if !config.allowed_listing_prices.contains(&price_rmi) {
            return Err(ContractError::InvalidListingPrice {});
        }

        // Resource bundle must not be empty
        if resources.len() == 0 {
            return Err(ContractError::EmptyResourceBundle {});
        }

        // Validate the resource bundle - valid resource IDs, no duplicates, integer amounts
        let mut seen: HashSet<String> = HashSet::new();
        for resource in resources.iter() {
            // Check resource ID exists
            validate_resource_id(&resource.id)?;

            // Check listed amount is an integer
            if resource.amount.u128() % RESOURCE_INTEGER_AMOUNT != 0 {
                return Err(ContractError::PartialResourceAmount(
                    resource.amount.clone(),
                ));
            }

            if seen.contains(&resource.id) {
                return Err(ContractError::DuplicateResourceId(resource.id.clone()));
            }

            seen.insert(resource.id.clone());
        }

        Ok(Listing {
            listing_id,
            lister_xyz_id,
            price_rmi,
            deposit_rmi_denom,
            deposit_rmi_amount: config.deposit_rmi_amount(price_rmi),
            created_at: timestamp,
            active_at: timestamp.plus_seconds(config.listing_pending_seconds),
            expired_at: timestamp.plus_seconds(config.listing_expiry_seconds),
            resources,
        })
    }

    pub fn state(&self, current_timestamp: Timestamp) -> ListingState {
        if self.expired_at <= current_timestamp {
            ListingState::Expired
        } else if self.active_at > current_timestamp {
            ListingState::Pending
        } else {
            ListingState::Active
        }
    }

    pub fn resource_tuples(&self) -> Vec<(String, String)> {
        self.resources
            .iter()
            .map(|resource| (resource.id.clone(), resource.amount.to_string()))
            .collect::<Vec<(String, String)>>()
    }
}

const DEFAULT_PAGINATION_LIMIT: u32 = 10;
const MAX_PAGINATION_LIMIT: u32 = 30;

pub struct Listings<'a> {
    listings: IndexedMap<'a, U64Key, Listing, ListingIndexes<'a>>,
    listings_pk: Item<'a, u64>,
}

impl Default for Listings<'static> {
    fn default() -> Self {
        Self::new("listings", "listings__lister", "listings_pk")
    }
}

impl<'a> Listings<'a> {
    pub fn new(
        listings_key: &'a str,
        listings_lister_key: &'a str,
        listings_pk_key: &'a str,
    ) -> Self {
        let listing_indexes = ListingIndexes {
            lister_xyz_id: MultiIndex::new(
                |o, k| (o.lister_xyz_id.clone(), k),
                listings_key,
                listings_lister_key,
            ),
        };

        Listings {
            listings: IndexedMap::new(listings_key, listing_indexes),
            listings_pk: Item::new(listings_pk_key),
        }
    }

    fn next_pk(&self, storage: &mut dyn Storage) -> Result<u64, ContractError> {
        let pk = 1 + self.listings_pk.load(storage).unwrap_or(0);
        self.listings_pk.save(storage, &pk)?;

        Ok(pk)
    }

    pub fn save_listing(
        &self,
        storage: &mut dyn Storage,
        lister_xyz_id: String,
        price_rmi: Uint128,
        deposit_rmi_denom: String,
        timestamp: Timestamp,
        resources: Vec<Resource>,
    ) -> Result<Listing, ContractError> {
        let config = CONFIG.load(storage)?;
        let listing_id = self.next_pk(storage)?;

        let listing = Listing::try_new(
            config,
            listing_id,
            lister_xyz_id,
            price_rmi,
            deposit_rmi_denom,
            timestamp,
            resources,
        )?;

        self.listings
            .update(storage, listing_id.into(), |old| match old {
                Some(_) => Err(ContractError::StorageConflict {}),
                None => Ok(listing.clone()),
            })?;

        Ok(listing)
    }

    pub fn remove_listing(
        &self,
        storage: &mut dyn Storage,
        listing_id: u64,
    ) -> Result<(), ContractError> {
        self.listings
            .remove(storage, listing_id.into())
            .map_err(ContractError::Std)
    }

    pub fn fetch_listing(&self, storage: &dyn Storage, listing_id: u64) -> StdResult<Listing> {
        self.listings.load(storage, listing_id.into())
    }

    pub fn fetch_active_listing(
        &self,
        storage: &dyn Storage,
        listing_id: u64,
        timestamp: Timestamp,
    ) -> Result<Listing, ContractError> {
        let listing = self.fetch_listing(storage, listing_id)?;

        if listing.state(timestamp) != ListingState::Active {
            return Err(ContractError::InactiveListing {});
        }

        Ok(listing)
    }

    pub fn fetch_listings(
        &self,
        storage: &dyn Storage,
        timestamp: Timestamp,
        lister_xyz_id: Option<String>,
        prices: Vec<Uint128>,
        resources: Vec<String>,
        include_inactive: bool,
        ascending: bool,
        start_after: Option<u64>,
        limit: Option<u32>,
    ) -> StdResult<Vec<Listing>> {
        let limit = limit
            .unwrap_or(DEFAULT_PAGINATION_LIMIT)
            .min(MAX_PAGINATION_LIMIT) as usize;
        let start_after = start_after.map(|s| Bound::exclusive(s.to_be_bytes()));
        let (start, end, order) = if ascending {
            (start_after, None, Order::Ascending)
        } else {
            (None, start_after, Order::Descending)
        };

        let resource_set = HashSet::<String>::from_iter(resources.iter().cloned());
        let listing_filter = |listing: &Listing| -> bool {
            // filter inactive listings if include_inactive is false
            if !include_inactive && listing.state(timestamp) != ListingState::Active {
                return false;
            }
            // filter listings that don't have one of the given prices
            if prices.len() > 0 && !prices.contains(&listing.price_rmi) {
                return false;
            }
            // filter listings whose resource bundles include all the given resources
            if resource_set.len() > 0 {
                let listing_resource_set = HashSet::<String>::from_iter(
                    listing
                        .resources
                        .iter()
                        .map(|r| r.id.clone())
                        .collect::<Vec<String>>(),
                );
                if !resource_set.is_subset(&listing_resource_set) {
                    return false;
                }
            }
            return true;
        };

        let range = if let Some(lister_xyz_id) = lister_xyz_id {
            self.listings
                .idx
                .lister_xyz_id
                .prefix(lister_xyz_id)
                .range(storage, start, end, order)
        } else {
            self.listings.range(storage, start, end, order)
        };

        let result: StdResult<Vec<_>> = range
            .map(|item| item.map(|(_, listing)| listing))
            .filter(|listing| match listing {
                Err(_) => false,
                Ok(listing) => listing_filter(listing),
            })
            .take(limit)
            .collect();

        Ok(result?)
    }
}

pub struct ListingIndexes<'a> {
    pub lister_xyz_id: MultiIndex<'a, (String, Vec<u8>), Listing>,
}

impl<'a> IndexList<Listing> for ListingIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Listing>> + '_> {
        let v: Vec<&dyn Index<Listing>> = vec![&self.lister_xyz_id];
        Box::new(v.into_iter())
    }
}
