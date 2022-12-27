use collectxyz_planet_metaverse::util::{fetch_nft_data};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};

use cw2::set_contract_version;
use collectxyz_experience::{
    BalanceResponse, XyzExperience, DownloadLogoResponse, EmbeddedLogo, Logo, LogoInfo,
    MarketingInfoResponse, MinterResponse, TokenInfoResponse,
};

use crate::allowances::{
    execute_burn_from, execute_decrease_allowance, execute_increase_allowance, query_allowance,
};
use crate::enumerable::{query_all_accounts, query_all_allowances};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{MinterData, XyzPlanetResourceInfo, BALANCES, LOGO, MARKETING_INFO, TOKEN_INFO};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:collectxyz-resources-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const LOGO_SIZE_CAP: usize = 5 * 1024;

/// Checks if data starts with XML preamble
fn verify_xml_preamble(data: &[u8]) -> Result<(), ContractError> {
    // The easiest way to perform this check would be just match on regex, however regex
    // compilation is heavy and probably not worth it.

    let preamble = data
        .split_inclusive(|c| *c == b'>')
        .next()
        .ok_or(ContractError::InvalidXmlPreamble {})?;

    const PREFIX: &[u8] = b"<?xml ";
    const POSTFIX: &[u8] = b"?>";

    if !(preamble.starts_with(PREFIX) && preamble.ends_with(POSTFIX)) {
        Err(ContractError::InvalidXmlPreamble {})
    } else {
        Ok(())
    }

    // Additionally attributes format could be validated as they are well defined, as well as
    // comments presence inside of preable, but it is probably not worth it.
}

/// Validates XML logo
fn verify_xml_logo(logo: &[u8]) -> Result<(), ContractError> {
    verify_xml_preamble(logo)?;

    if logo.len() > LOGO_SIZE_CAP {
        Err(ContractError::LogoTooBig {})
    } else {
        Ok(())
    }
}

/// Validates png logo
fn verify_png_logo(logo: &[u8]) -> Result<(), ContractError> {
    // PNG header format:
    // 0x89 - magic byte, out of ASCII table to fail on 7-bit systems
    // "PNG" ascii representation
    // [0x0d, 0x0a] - dos style line ending
    // 0x1a - dos control character, stop displaying rest of the file
    // 0x0a - unix style line ending
    const HEADER: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
    if logo.len() > LOGO_SIZE_CAP {
        Err(ContractError::LogoTooBig {})
    } else if !logo.starts_with(&HEADER) {
        Err(ContractError::InvalidPngHeader {})
    } else {
        Ok(())
    }
}

/// Checks if passed logo is correct, and if not, returns an error
fn verify_logo(logo: &Logo) -> Result<(), ContractError> {
    match logo {
        Logo::Embedded(EmbeddedLogo::Svg(logo)) => verify_xml_logo(&logo),
        Logo::Embedded(EmbeddedLogo::Png(logo)) => verify_png_logo(&logo),
        Logo::Url(_) => Ok(()), // Any reasonable url validation would be regex based, probably not worth it
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;
    // create initial accounts
    let total_supply = create_accounts(
        &mut deps, &msg.initial_balances, &msg.xyz_contract_address
    )?;

    if let Some(limit) = msg.get_caps(info.sender.to_string()) {
        if total_supply > limit {
            return Err(StdError::generic_err("Initial supply greater than cap").into());
        }
    }

    let mints: Option<Vec<MinterData>> = match msg.mint {
        Some(m) => Some(
            m.iter().map(|mr| 
                Ok(
                    MinterData {
                        minter: deps.api.addr_validate(&mr.minter)?,
                        cap: mr.cap,
                    }
                )
            ).collect::<StdResult<Vec<MinterData>>>()?
        ),
        None => None,
    };

    // store token info
    let data = XyzPlanetResourceInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply,
        minters: mints,
        xyz_contract_address: msg.xyz_contract_address,
    };
    TOKEN_INFO.save(deps.storage, &data)?;

    if let Some(marketing) = msg.marketing {
        let logo = if let Some(logo) = marketing.logo {
            verify_logo(&logo)?;
            LOGO.save(deps.storage, &logo)?;

            match logo {
                Logo::Url(url) => Some(LogoInfo::Url(url)),
                Logo::Embedded(_) => Some(LogoInfo::Embedded),
            }
        } else {
            None
        };

        let data = MarketingInfoResponse {
            project: marketing.project,
            description: marketing.description,
            marketing: marketing
                .marketing
                .map(|addr| deps.api.addr_validate(&addr))
                .transpose()?,
            logo,
        };
        MARKETING_INFO.save(deps.storage, &data)?;
    }

    Ok(Response::default())
}

pub fn create_accounts(deps: &mut DepsMut, accounts: &[XyzExperience], xyz_contract_addr: &Addr) -> StdResult<Uint128> {
    let mut total_supply = Uint128::zero();
    for row in accounts {
        // validate that nft_id exists
        let _ = fetch_nft_data(
            &row.xyz_id, &xyz_contract_addr, &deps.querier
        )?;
        BALANCES.save(deps.storage, &row.xyz_id, &row.amount)?;
        total_supply += row.amount;
    }
    Ok(total_supply)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { recipient_xyz_id, amount } => execute_mint(deps, env, info, recipient_xyz_id, amount),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
            owner_xyz_id,
        } => execute_increase_allowance(deps, env, info, spender, amount, expires, owner_xyz_id),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
            owner_xyz_id,
        } => execute_decrease_allowance(deps, env, info, spender, amount, expires, owner_xyz_id),
        ExecuteMsg::BurnFrom { owner, amount, owner_xyz_id } => execute_burn_from(deps, env, info, owner, owner_xyz_id, amount),
        ExecuteMsg::UpdateMarketing {
            project,
            description,
            marketing,
        } => execute_update_marketing(deps, env, info, project, description, marketing),
        ExecuteMsg::UploadLogo(logo) => execute_upload_logo(deps, env, info, logo),
    }
}

pub fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient_xyz_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let mut config = TOKEN_INFO.load(deps.storage)?;
    let minter = config.get_minter(info.sender.to_string());
    if minter.is_none() || minter.unwrap().minter != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // update supply and enforce cap
    config.total_supply += amount;
    if let Some(limit) = config.get_cap(info.sender.to_string()) {
        if config.total_supply > limit {
            return Err(ContractError::CannotExceedCap {});
        }
    }
    TOKEN_INFO.save(deps.storage, &config)?;

    // validate that the nft id is valid  
    let _ = fetch_nft_data(
        &recipient_xyz_id,
        &config.xyz_contract_address,
        &deps.querier
    )?;
    
    // add amount to recipient balance
    BALANCES.update(
        deps.storage,
        &recipient_xyz_id,
        |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + amount) },
    )?;

    let res = Response::new()
        .add_attribute("action", "mint")
        .add_attribute("to_xyz_id", recipient_xyz_id)
        .add_attribute("amount", amount);
    Ok(res)
}

pub fn execute_update_marketing(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    project: Option<String>,
    description: Option<String>,
    marketing: Option<String>,
) -> Result<Response, ContractError> {
    let mut marketing_info = MARKETING_INFO
        .may_load(deps.storage)?
        .ok_or(ContractError::Unauthorized {})?;

    if marketing_info
        .marketing
        .as_ref()
        .ok_or(ContractError::Unauthorized {})?
        != &info.sender
    {
        return Err(ContractError::Unauthorized {});
    }

    match project {
        Some(empty) if empty.trim().is_empty() => marketing_info.project = None,
        Some(project) => marketing_info.project = Some(project),
        None => (),
    }

    match description {
        Some(empty) if empty.trim().is_empty() => marketing_info.description = None,
        Some(description) => marketing_info.description = Some(description),
        None => (),
    }

    match marketing {
        Some(empty) if empty.trim().is_empty() => marketing_info.marketing = None,
        Some(marketing) => marketing_info.marketing = Some(deps.api.addr_validate(&marketing)?),
        None => (),
    }

    if marketing_info.project.is_none()
        && marketing_info.description.is_none()
        && marketing_info.marketing.is_none()
        && marketing_info.logo.is_none()
    {
        MARKETING_INFO.remove(deps.storage);
    } else {
        MARKETING_INFO.save(deps.storage, &marketing_info)?;
    }

    let res = Response::new().add_attribute("action", "update_marketing");
    Ok(res)
}

pub fn execute_upload_logo(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    logo: Logo,
) -> Result<Response, ContractError> {
    let mut marketing_info = MARKETING_INFO
        .may_load(deps.storage)?
        .ok_or(ContractError::Unauthorized {})?;

    verify_logo(&logo)?;

    if marketing_info
        .marketing
        .as_ref()
        .ok_or(ContractError::Unauthorized {})?
        != &info.sender
    {
        return Err(ContractError::Unauthorized {});
    }

    LOGO.save(deps.storage, &logo)?;

    let logo_info = match logo {
        Logo::Url(url) => LogoInfo::Url(url),
        Logo::Embedded(_) => LogoInfo::Embedded,
    };

    marketing_info.logo = Some(logo_info);
    MARKETING_INFO.save(deps.storage, &marketing_info)?;

    let res = Response::new().add_attribute("action", "upload_logo");
    Ok(res)
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let mut token_info = TOKEN_INFO.load(deps.storage)?;
    token_info.minters = Some(msg.minters);
    TOKEN_INFO.save(deps.storage, &token_info)?;
    return Ok(Response::new().add_attribute("method", "migrate"));
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { xyz_id } => to_binary(&query_balance(deps, xyz_id)?),
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Minter {} => to_binary(&query_minters(deps)?),
        QueryMsg::Allowance { owner_xyz_id, owner, spender } => {
            to_binary(&query_allowance(deps, owner, owner_xyz_id, spender)?)
        }
        QueryMsg::AllAllowances {
            owner_xyz_id,
            owner,
            start_after,
            limit,
        } => to_binary(&query_all_allowances(deps, owner, owner_xyz_id, start_after, limit)?),
        QueryMsg::AllAccounts { start_after, limit } => {
            to_binary(&query_all_accounts(deps, start_after, limit)?)
        }
        QueryMsg::MarketingInfo {} => to_binary(&query_marketing_info(deps)?),
        QueryMsg::DownloadLogo {} => to_binary(&query_download_logo(deps)?),
    }
}

pub fn query_balance(deps: Deps, xyz_id: String) -> StdResult<BalanceResponse> {
    let balance = BALANCES
        .may_load(deps.storage, &xyz_id)?
        .unwrap_or_default();
    Ok(BalanceResponse { balance })
}

pub fn query_token_info(deps: Deps) -> StdResult<TokenInfoResponse> {
    let info = TOKEN_INFO.load(deps.storage)?;
    let res = TokenInfoResponse {
        name: info.name,
        symbol: info.symbol,
        decimals: info.decimals,
        total_supply: info.total_supply,
    };
    Ok(res)
}

pub fn query_minters(deps: Deps) -> StdResult<Option<Vec<MinterResponse>>> {
    let meta = TOKEN_INFO.load(deps.storage)?;
    let minter = match meta.minters {
        Some(m) => Some(
            m.iter().map(|v|
                MinterResponse {
                    minter: v.minter.clone().into(),
                    cap: v.cap,
                }
            ).collect()
        ),
        None => None,
    };
    return Ok(minter);
}

pub fn query_marketing_info(deps: Deps) -> StdResult<MarketingInfoResponse> {
    Ok(MARKETING_INFO.may_load(deps.storage)?.unwrap_or_default())
}

pub fn query_download_logo(deps: Deps) -> StdResult<DownloadLogoResponse> {
    let logo = LOGO.load(deps.storage)?;
    match logo {
        Logo::Embedded(EmbeddedLogo::Svg(logo)) => Ok(DownloadLogoResponse {
            mime_type: "image/svg+xml".to_owned(),
            data: logo,
        }),
        Logo::Embedded(EmbeddedLogo::Png(logo)) => Ok(DownloadLogoResponse {
            mime_type: "image/png".to_owned(),
            data: logo,
        }),
        Logo::Url(_) => Err(StdError::not_found("logo")),
    }
}
