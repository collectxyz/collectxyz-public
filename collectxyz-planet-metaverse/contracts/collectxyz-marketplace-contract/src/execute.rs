use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg};

use collectxyz_planet_metaverse::util::{
    burn_resource, check_sufficient_funds, mint_resource, validate_nft_is_owned_by_wallet,
};

use crate::error::ContractError;
use crate::msg::{ConfigPatch, InstantiateMsg, MigrateMsg};
use crate::state::{Config, Listings, Resource, CONFIG, OWNER};

pub fn execute_instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    OWNER.save(deps.storage, &info.sender)?;
    CONFIG.save(deps.storage, &msg.config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

pub fn execute_make_listing(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    lister_xyz_id: String,
    price_rmi: Uint128,
    deposit_rmi_denom: String,
    resources: Vec<Resource>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // check that the sender owns lister_xyz_id
    if !validate_nft_is_owned_by_wallet(
        &lister_xyz_id,
        &info.sender.to_string(),
        &deps.querier,
        &config.xyz_nft_contract,
    )? {
        return Err(ContractError::Unauthorized {});
    };

    // check that the sender provided sufficient make listing fee
    check_sufficient_funds(info.funds, &config.make_listing_fee)?;

    // save the listing to storage
    let listings = Listings::default();
    let listing = listings.save_listing(
        deps.storage,
        lister_xyz_id.clone(),
        price_rmi,
        deposit_rmi_denom.clone(),
        env.block.time,
        resources.clone(),
    )?;

    // burn listed resources from lister_xyz_id
    let mut messages: Vec<WasmMsg> = vec![];
    for resource in resources.iter() {
        let resource_contract = config.resource_addr(&resource.id)?;
        messages.push(burn_resource(
            info.sender.to_string(),
            lister_xyz_id.clone(),
            resource_contract.to_string(),
            resource.amount,
        )?)
    }

    // burn listing RMI deposit from lister_xyz_id
    let rmi_denom_contract = config.resource_addr(&deposit_rmi_denom)?;
    if !listing.deposit_rmi_amount.is_zero() {
        messages.push(burn_resource(
            info.sender.to_string(),
            lister_xyz_id.clone(),
            rmi_denom_contract.to_string(),
            listing.deposit_rmi_amount,
        )?);
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("method", "execute")
        .add_attribute("action", "make_listing")
        .add_attribute("listing", serde_json::to_string(&listing).unwrap()))
}

pub fn execute_revoke_listing(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    listing_id: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let listings = Listings::default();
    let listing = listings.fetch_listing(deps.storage, listing_id)?;

    // check that the sender owns listing.lister_xyz_id
    if !validate_nft_is_owned_by_wallet(
        &listing.lister_xyz_id,
        &info.sender.to_string(),
        &deps.querier,
        &config.xyz_nft_contract,
    )? {
        return Err(ContractError::Unauthorized {});
    };

    // remove the listing from storage
    listings.remove_listing(deps.storage, listing_id)?;

    // mint listed resources back to lister_xyz_id
    let mut messages: Vec<WasmMsg> = vec![];
    for resource in listing.resources.iter() {
        let resource_contract = config.resource_addr(&resource.id)?;
        messages.push(mint_resource(
            listing.lister_xyz_id.clone(),
            resource_contract.to_string(),
            resource.amount,
        )?)
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("method", "execute")
        .add_attribute("action", "revoke_listing")
        .add_attribute("listing", serde_json::to_string(&listing).unwrap()))
}

pub fn execute_take_listing(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    listing_id: u64,
    taker_xyz_id: String,
    rmi_denom: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // check that the sender owns taker_xyz_id
    if !validate_nft_is_owned_by_wallet(
        &taker_xyz_id,
        &info.sender.to_string(),
        &deps.querier,
        &config.xyz_nft_contract,
    )? {
        return Err(ContractError::Unauthorized {});
    };

    // check that the sender provided sufficient take listing fee
    check_sufficient_funds(info.funds, &config.take_listing_fee)?;

    let listings = Listings::default();

    // check that the listing is active
    let listing = listings.fetch_active_listing(deps.storage, listing_id, env.block.time)?;

    // block self-trades, since otherwise this would allow people to remove an listing
    // from the marketplace but get back their RMI deposit
    if taker_xyz_id == listing.lister_xyz_id {
        return Err(ContractError::CantTakeOwnListing {});
    }

    // remove the listing from storage
    listings.remove_listing(deps.storage, listing.listing_id)?;

    let mut messages: Vec<WasmMsg> = vec![];
    let rmi_denom_contract = config.resource_addr(&rmi_denom)?;

    // for lister_xyz_id: mint listing.price_rmi in rmi_denom
    messages.push(mint_resource(
        listing.lister_xyz_id.clone(),
        rmi_denom_contract.to_string(),
        listing.price_rmi,
    )?);

    // for taker_xyz_id: burn listing.price_rmi in rmi_denom
    // NOTE: this burn operation will fail if taker_xyz_id has insufficient balance of rmi_denom.
    // we rely on this to failure that people can't buy if they don't have enough funds.
    messages.push(burn_resource(
        info.sender.to_string(),
        taker_xyz_id.clone(),
        rmi_denom_contract.to_string(),
        listing.price_rmi,
    )?);

    // for lister_xyz_id: mint deposit_rmi_amount in listing.deposit_rmi_denom
    if !listing.deposit_rmi_amount.is_zero() {
        messages.push(mint_resource(
            listing.lister_xyz_id.clone(),
            config
                .resource_addr(&listing.deposit_rmi_denom)?
                .to_string(),
            listing.deposit_rmi_amount,
        )?);
    }

    // for taker_xyz_id: mint listing.resources
    for resource in listing.resources.iter() {
        let resource_contract = config.resource_addr(&resource.id)?;
        messages.push(mint_resource(
            taker_xyz_id.clone(),
            resource_contract.to_string(),
            resource.amount,
        )?)
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("method", "execute")
        .add_attribute("action", "take_listing")
        .add_attribute("taker_xyz_id", taker_xyz_id)
        .add_attribute("listing", serde_json::to_string(&listing).unwrap()))
}

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    config_patch: ConfigPatch,
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;

    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    let config = CONFIG.load(deps.storage)?;

    let new_config = Config {
        listing_expiry_seconds: config_patch
            .listing_expiry_seconds
            .unwrap_or(config.listing_expiry_seconds),
        listing_pending_seconds: config_patch
            .listing_pending_seconds
            .unwrap_or(config.listing_pending_seconds),
        listing_deposit_percent: config_patch
            .listing_deposit_percent
            .unwrap_or(config.listing_deposit_percent),
        allowed_listing_prices: config_patch
            .allowed_listing_prices
            .unwrap_or(config.allowed_listing_prices),
        make_listing_fee: config_patch
            .make_listing_fee
            .unwrap_or(config.make_listing_fee),
        take_listing_fee: config_patch
            .take_listing_fee
            .unwrap_or(config.take_listing_fee),
        xyz_nft_contract: config_patch
            .xyz_nft_contract
            .unwrap_or(config.xyz_nft_contract),
        rock_contract: config_patch.rock_contract.unwrap_or(config.rock_contract),
        ice_contract: config_patch.ice_contract.unwrap_or(config.ice_contract),
        metal_contract: config_patch.metal_contract.unwrap_or(config.metal_contract),
        gas_contract: config_patch.gas_contract.unwrap_or(config.gas_contract),
        water_contract: config_patch.water_contract.unwrap_or(config.water_contract),
        gem_contract: config_patch.gem_contract.unwrap_or(config.gem_contract),
        life_contract: config_patch.life_contract.unwrap_or(config.life_contract),
    };

    CONFIG.save(deps.storage, &new_config)?;

    Ok(Response::new()
        .add_attribute("method", "execute")
        .add_attribute("action", "update_config"))
}

pub fn execute_migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> StdResult<Response> {
    CONFIG.save(deps.storage, &msg.config)?;

    Ok(Response::new().add_attribute("method", "migrate"))
}
