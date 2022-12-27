use cosmwasm_std::{BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Storage, entry_point, to_binary};
use cw2::set_contract_version;
use std::str;

use crate::planet_util::{query_all_planets_for_coord};
use crate::complete_task::try_claim;
use crate::start_task::{query_task_for_nft, try_start_task};

use collectxyz_planet_metaverse::discover_planets::QueryMsg;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, ResourceGenerationInfo, UpdateConfigData};
use crate::state::{ADMIN, CONFIG, Config, save};

const CONTRACT_NAME: &str = "crates.io:xyz-planet-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        probability_of_discovery: msg.probability_of_discovery,
        required_seconds: msg.required_seconds,
        resource_generation_info: msg.resource_generation_info,
        core_resource_generation_info: msg.core_resource_generation_info,
        maximum_planets_per_coord: msg.maximum_planets_per_coord,
        randomness_contract_address: msg.randomness_contract_address,
        xyz_nft_contract_address: msg.xyz_nft_contract_address,
        discovery_task_expiration_window_seconds: msg.discovery_task_expiration_window_seconds,
        max_number_of_bonus_tokens: msg.max_number_of_bonus_tokens,
        boost_per_bonus_token: msg.boost_per_bonus_token,
        cw20_bonus_token_contract: msg.cw20_bonus_token_contract,
        start_task_fee: msg.start_task_fee,
        experience_mint_config: msg.experience_mint_config.into(),
    };

    update_resource_contract_lookup(deps.storage, &config.resource_generation_info)?;
    update_resource_contract_lookup(deps.storage, &config.core_resource_generation_info)?;

    CONFIG.save(deps.storage, &config)?;
    let admin = &info.sender;
    ADMIN.save(deps.storage, &admin)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::StartTask { xyz_nft_id, bonus_token_count } => try_start_task(
            xyz_nft_id,
            info,
            bonus_token_count,
            deps.storage,
            env.block,
            &deps.querier,
        ),
        ExecuteMsg::CompleteTask { xyz_nft_id } => try_claim(
            xyz_nft_id,
            info.sender.to_string(),
            &deps.querier,
            deps.storage,
            env.block,
        ),
        ExecuteMsg::UpdateConfig { update_config_data } => {
            update_config(info.sender.to_string(), deps.storage, update_config_data)
        },
        ExecuteMsg::Withdraw { amount } => execute_withdraw(deps, env, info, amount),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCurrentConfig {} => to_binary(&query_config(deps.storage)?),
        QueryMsg::GetTaskForNft { xyz_nft_id } => {
            to_binary(&query_task_for_nft(deps.storage, &xyz_nft_id)?)
        }
        QueryMsg::GetPlanetsForCoords {
            coordinates,
            start_after,
            limit,
        } => to_binary(&query_all_planets_for_coord(
            deps.storage,
            &coordinates,
            start_after,
            limit,
        )?),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    return Ok(Response::default())
}

pub fn execute_withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Vec<Coin>,
) -> Result<Response, StdError> {
    let admin = ADMIN.load(deps.storage)?;
    if !info.sender.eq(&admin) {
        return Err(StdError::generic_err("Only admin can execute this method."));
    }

    Ok(Response::new().add_message(BankMsg::Send {
        amount,
        to_address: admin.to_string(),
    }))
}

fn update_resource_contract_lookup(
    storage: &mut dyn Storage,
    infos: &Vec<ResourceGenerationInfo>,
) -> StdResult<()> {
    for resource in infos {
        save(
            storage,
            &resource.resource_identifier,
            &resource.resource_contract_address,
        )?;
    }
    return Ok(());
}

fn update_config(
    sender: String,
    storage: &mut dyn Storage,
    update_config_data: UpdateConfigData,
) -> Result<Response, StdError> {
    let admin_result = ADMIN.load(storage);
    if admin_result.is_err() {
        return Err(admin_result.unwrap_err());
    }

    let admin: String = admin_result.unwrap().to_string();
    if !sender.eq(&admin) {
        return Err(StdError::generic_err("Only admin can execute this method."));
    }

    let config_result = CONFIG.load(storage);

    if config_result.is_err() {
        return Err(config_result.unwrap_err());
    }

    let config = config_result.unwrap();

    let _resource_generation_info = update_config_data.resource_generation_info.clone();
    let _core_resource_generation_info = update_config_data.core_resource_generation_info.clone();

    let updated_config = Config {
        probability_of_discovery: if update_config_data.probability_of_discovery.is_some() {
            update_config_data.probability_of_discovery.unwrap()
        } else {
            config.probability_of_discovery
        },

        required_seconds: update_config_data
            .required_seconds
            .unwrap_or(config.required_seconds),

        maximum_planets_per_coord: update_config_data
            .maximum_planets_per_coord
            .unwrap_or(config.maximum_planets_per_coord),

        randomness_contract_address: update_config_data
            .randomness_contract_address
            .unwrap_or(config.randomness_contract_address),

        xyz_nft_contract_address: update_config_data
            .xyz_nft_contract_address
            .unwrap_or(config.xyz_nft_contract_address),

        resource_generation_info: update_config_data
            .resource_generation_info
            .unwrap_or(config.resource_generation_info),

        core_resource_generation_info: update_config_data
            .core_resource_generation_info
            .unwrap_or(config.core_resource_generation_info),

        discovery_task_expiration_window_seconds: update_config_data
            .discovery_task_expiration_window_seconds
            .unwrap_or(config.discovery_task_expiration_window_seconds),

        max_number_of_bonus_tokens: update_config_data
            .max_number_of_bonus_tokens
            .unwrap_or(config.max_number_of_bonus_tokens),

        boost_per_bonus_token: update_config_data
            .boost_per_bonus_token
            .unwrap_or(config.boost_per_bonus_token),

        cw20_bonus_token_contract: update_config_data
            .cw20_bonus_token_contract
            .unwrap_or(config.cw20_bonus_token_contract),
        
        start_task_fee: update_config_data
            .start_task_fee
            .unwrap_or(config.start_task_fee),

        experience_mint_config: update_config_data
            .experience_mint_config
            .unwrap_or(config.experience_mint_config.into()).into(),
    };

    if _resource_generation_info.is_some() {
        update_resource_contract_lookup(storage, &_resource_generation_info.unwrap())?
    }

    if _core_resource_generation_info.is_some() {
        update_resource_contract_lookup(storage, &_core_resource_generation_info.unwrap())?
    }

    CONFIG.save(storage, &updated_config)?;
    return Ok(Response::default());
}

pub fn query_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::{InstantiateMsg, ResourceGenerationInfo};
    use crate::state::{TASK_REPOSITORY};
    use collectxyz_planet_metaverse::experience::XyzExperienceMintInfo;
    use collectxyz_planet_metaverse::mock_querier::{EXPERIENCE_CONTRACT_ADDRESS, mock_dependencies_custom};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr, Coin, Uint128};

    use crate::test_helpers::{DEFAULT_BOOST_PER_BONUS_TOKEN, DEFAULT_CW20_BONUS_TOKEN_CONTRACT, DEFAULT_DISCOVERY_EXPIRATION_WINDOW, DEFAULT_MAX_BONUS_TOKEN_COUNT, XYZ_NFT_ID, default_resource_generation_info_with_id, default_task};

    use crate::test_helpers::{
        default_resource_generation_info, EIGHTY_PERCENT, MAX_ALLOWED_PLANETS, TWO_DAYS,
    };

    #[test]
    fn proper_init() {
        let mut deps = mock_dependencies(&[]);
        let resource_gen_config: Vec<ResourceGenerationInfo> = vec![
            default_resource_generation_info("test1"),
            default_resource_generation_info("test2"),
        ];

        let core_resource_gen_config: Vec<ResourceGenerationInfo> = vec![
            default_resource_generation_info("test1"),
            default_resource_generation_info("test2"),
        ];

        let experience_config = XyzExperienceMintInfo {
            experience_contract_address: Addr::unchecked(EXPERIENCE_CONTRACT_ADDRESS.to_string()),
            complete_task_experience_amount: Uint128::from(0u128),
        };

        let init_msg = InstantiateMsg {
            probability_of_discovery: EIGHTY_PERCENT,
            required_seconds: TWO_DAYS,
            resource_generation_info: resource_gen_config,
            core_resource_generation_info: core_resource_gen_config,
            maximum_planets_per_coord: MAX_ALLOWED_PLANETS,
            randomness_contract_address: Addr::unchecked("random"),
            xyz_nft_contract_address: Addr::unchecked("nft_address"),
            discovery_task_expiration_window_seconds: DEFAULT_DISCOVERY_EXPIRATION_WINDOW,
            max_number_of_bonus_tokens: DEFAULT_MAX_BONUS_TOKEN_COUNT,
            boost_per_bonus_token: DEFAULT_BOOST_PER_BONUS_TOKEN,
            cw20_bonus_token_contract: Addr::unchecked(DEFAULT_CW20_BONUS_TOKEN_CONTRACT),
            start_task_fee: Coin::new(100, "uluna"),
            experience_mint_config: experience_config,
        };
        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("addr0002", &[]),
            init_msg,
        )
        .unwrap();
    }

    #[test]
    fn test_update_config_no_change() {
        let mut deps = mock_dependencies_custom(
            None,
            None,
            None,
            vec![],
            &[Coin {
                denom: "ust".to_string(),
                amount: Uint128::new(9_000_000),
            }],
        );
        let deps_mut = deps.as_mut();

        let experience_config = XyzExperienceMintInfo {
            experience_contract_address: Addr::unchecked(EXPERIENCE_CONTRACT_ADDRESS.to_string()),
            complete_task_experience_amount: Uint128::from(0u128),
        };

        let config = Config {
            probability_of_discovery: 255, // Highest probability
            required_seconds: TWO_DAYS,
            resource_generation_info: vec![default_resource_generation_info("test")],
            core_resource_generation_info: vec![default_resource_generation_info("test")],
            maximum_planets_per_coord: 10,
            randomness_contract_address: Addr::unchecked("random"),
            xyz_nft_contract_address: Addr::unchecked("nft_address"),
            discovery_task_expiration_window_seconds: DEFAULT_DISCOVERY_EXPIRATION_WINDOW,
            max_number_of_bonus_tokens: DEFAULT_MAX_BONUS_TOKEN_COUNT,
            boost_per_bonus_token: DEFAULT_BOOST_PER_BONUS_TOKEN,
            cw20_bonus_token_contract: Addr::unchecked(DEFAULT_CW20_BONUS_TOKEN_CONTRACT),
            start_task_fee: Coin::new(100, "uluna"),
            experience_mint_config: experience_config.into(),
        };
        let _ = CONFIG.save(deps_mut.storage, &config);
        let addr = Addr::unchecked("admin");
        let _ = ADMIN.save(deps_mut.storage, &addr);

        let update = UpdateConfigData {
            probability_of_discovery: None,
            required_seconds: None,
            resource_generation_info: None,
            core_resource_generation_info: None,
            maximum_planets_per_coord: None,
            randomness_contract_address: None,
            xyz_nft_contract_address: None,
            discovery_task_expiration_window_seconds: None,
            max_number_of_bonus_tokens: None,
            boost_per_bonus_token: None,
            cw20_bonus_token_contract: None,
            start_task_fee: None,
            experience_mint_config: None,
        };

        let update_result = update_config(addr.to_string(), deps_mut.storage, update);

        assert_eq!(update_result.is_ok(), true);
        let config_result = CONFIG.load(deps_mut.storage);
        assert_eq!(config_result.unwrap(), config)
    }

    #[test]
    fn test_update_config_fails_for_non_admin() {
        let mut deps = mock_dependencies_custom(
            None,
            None,
            None,
            vec![],
            &[Coin {
                denom: "ust".to_string(),
                amount: Uint128::new(9_000_000),
            }],
        );
        let deps_mut = deps.as_mut();

        let experience_config = XyzExperienceMintInfo {
            experience_contract_address: Addr::unchecked(EXPERIENCE_CONTRACT_ADDRESS.to_string()),
            complete_task_experience_amount: Uint128::from(0u128),
        };

        let config = Config {
            probability_of_discovery: 255, // Highest probability
            required_seconds: TWO_DAYS,
            resource_generation_info: vec![default_resource_generation_info("test")],
            core_resource_generation_info: vec![default_resource_generation_info("test")],
            maximum_planets_per_coord: 10,
            randomness_contract_address: Addr::unchecked("random"),
            xyz_nft_contract_address: Addr::unchecked("nft_address"),
            discovery_task_expiration_window_seconds: DEFAULT_DISCOVERY_EXPIRATION_WINDOW,
            max_number_of_bonus_tokens: DEFAULT_MAX_BONUS_TOKEN_COUNT,
            boost_per_bonus_token: DEFAULT_BOOST_PER_BONUS_TOKEN,
            cw20_bonus_token_contract: Addr::unchecked(DEFAULT_CW20_BONUS_TOKEN_CONTRACT),
            start_task_fee: Coin::new(100, "uluna"),
            experience_mint_config: experience_config.into(),
        };
        let _ = CONFIG.save(deps_mut.storage, &config);
        let admin = Addr::unchecked("admin");
        let _ = ADMIN.save(deps_mut.storage, &admin);

        let non_admin = Addr::unchecked("nonAdmin");

        let update = UpdateConfigData {
            probability_of_discovery: None,
            required_seconds: None,
            resource_generation_info: None,
            core_resource_generation_info: None,
            maximum_planets_per_coord: None,
            randomness_contract_address: None,
            xyz_nft_contract_address: None,
            discovery_task_expiration_window_seconds: None,
            max_number_of_bonus_tokens: None,
            boost_per_bonus_token: None,
            cw20_bonus_token_contract: None,
            start_task_fee: None,
            experience_mint_config: None,
        };

        let update_result = update_config(non_admin.to_string(), deps_mut.storage, update);

        assert_eq!(update_result.is_err(), true);
        assert_eq!(
            update_result.unwrap_err(),
            StdError::generic_err("Only admin can execute this method.")
        );
    }

    #[test]
    fn test_get_nft_discovery_success() {
        let mut deps = mock_dependencies_custom(
            None,
            None,
            None,
            vec![],
            &[Coin {
                denom: "ust".to_string(),
                amount: Uint128::new(9_000_000),
            }],
        );
        let env = mock_env();
        let deps_mut = deps.as_mut();

        let discover_info = default_task();

        let _ = TASK_REPOSITORY.save_task(deps_mut.storage, &discover_info);

        let discovery = query(
            deps_mut.as_ref(),
            env,
            QueryMsg::GetTaskForNft {
                xyz_nft_id: XYZ_NFT_ID.to_string(),
            },
        );

        assert_eq!(discovery.is_ok(), true);
    }

    #[test]
    fn test_resource_lookup_save_and_load() {
        let mut deps = mock_dependencies_custom(
            None,
            None,
            None,
            vec![],
            &[Coin {
                denom: "ust".to_string(),
                amount: Uint128::new(9_000_000),
            }],
        );
        let deps_mut = deps.as_mut();

        let core_resources = vec![
            default_resource_generation_info_with_id("core_id1"),
            default_resource_generation_info_with_id("core_id2"),
        ];

        let non_core_resources = vec![
            default_resource_generation_info_with_id("non_core_id1"),
            default_resource_generation_info_with_id("non_core_id2"),
        ];

        // Core
        let result = update_resource_contract_lookup(deps_mut.storage, &core_resources);
        assert_eq!(result.is_err(), false);

        // Non core
        let result = update_resource_contract_lookup(deps_mut.storage, &non_core_resources);
        assert_eq!(result.is_err(), false);
    }
}
