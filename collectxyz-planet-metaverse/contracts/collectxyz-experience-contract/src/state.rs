use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use collectxyz_experience::{AllowanceResponse, Logo, MarketingInfoResponse};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct XyzPlanetResourceInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub minters: Option<Vec<MinterData>>,
    pub xyz_contract_address: Addr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MinterData {
    pub minter: Addr,
    /// cap is how many more tokens can be issued by the minter
    pub cap: Option<Uint128>,
}

impl XyzPlanetResourceInfo {
    pub fn get_cap(&self, sender: String) -> Option<Uint128> {
        return self.minters.as_ref().and_then(|v| 
            v.iter().find(|minter| minter.minter.eq(&sender)).and_then(|m| m.cap)
        );
    }

    pub fn get_minter(&self, sender: String) -> Option<&MinterData> {
        return self.minters.as_ref().and_then(|v|
            v.iter().find(|minter| minter.minter.eq(&sender))
        );
    }
}

pub const TOKEN_INFO: Item<XyzPlanetResourceInfo> = Item::new("token_info");
pub const MARKETING_INFO: Item<MarketingInfoResponse> = Item::new("marketing_info");
pub const LOGO: Item<Logo> = Item::new("logo");
/// Balances are mapped by XYZ ID
pub const BALANCES: Map<&str, Uint128> = Map::new("balance");
/// Allowance is granted for xyz_id,xyz_owner -> some_addr
pub const ALLOWANCES: Map<(&str, &Addr, &Addr), AllowanceResponse> = Map::new("allowance");