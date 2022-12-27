use collectxyz_planet_metaverse::util::{validate_nft_is_owned_by_wallet};
use cosmwasm_std::{
    attr, Addr, BlockInfo, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Storage, Uint128,
};
use collectxyz_experience::{AllowanceResponse, Expiration};

use crate::error::ContractError;
use crate::state::{ALLOWANCES, BALANCES, TOKEN_INFO};

pub fn execute_increase_allowance(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    spender: String,
    amount: Uint128,
    expires: Option<Expiration>,
    owner_xyz_id: String,
) -> Result<Response, ContractError> {
    let  config = TOKEN_INFO.load(deps.storage)?;
    let owner = &info.sender;
    // Validate owner_xyz_id is owned by owner
    if !validate_nft_is_owned_by_wallet(
        &owner_xyz_id,
        &owner.to_string(),
        &deps.querier,
        &config.xyz_contract_address
    )? {
        return Err(ContractError::Unauthorized {});
    }

    let spender_addr = deps.api.addr_validate(&spender)?;
    if spender_addr == info.sender {
        return Err(ContractError::CannotSetOwnAccount {});
    }

    ALLOWANCES.update(
        deps.storage,
        (&owner_xyz_id, &info.sender, &spender_addr),
        |allow| -> StdResult<_> {
            let mut val = allow.unwrap_or_default();
            if let Some(exp) = expires {
                val.expires = exp;
            }
            val.allowance += amount;
            Ok(val)
        },
    )?;

    let res = Response::new().add_attributes(vec![
        attr("action", "increase_allowance"),
        attr("owner", info.sender),
        attr("owner_xyz_id", owner_xyz_id),
        attr("spender", spender),
        attr("amount", amount),
    ]);
    Ok(res)
}

pub fn execute_decrease_allowance(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    spender: String,
    amount: Uint128,
    expires: Option<Expiration>,
    owner_xyz_id: String,
) -> Result<Response, ContractError> {
    let  config = TOKEN_INFO.load(deps.storage)?;
    let owner = &info.sender;
    // Validate owner_xyz_id is owned by owner
    if !validate_nft_is_owned_by_wallet(
        &owner_xyz_id,
        &owner.to_string(),
        &deps.querier,
        &config.xyz_contract_address
    )? {
        return Err(ContractError::Unauthorized {});
    }

    let spender_addr = deps.api.addr_validate(&spender)?;
    if spender_addr == info.sender {
        return Err(ContractError::CannotSetOwnAccount {});
    }

    let key = (owner_xyz_id.as_str(), owner, &spender_addr);
    // load value and delete if it hits 0, or update otherwise
    let mut allowance = ALLOWANCES.load(deps.storage, key)?;
    if amount < allowance.allowance {
        // update the new amount
        allowance.allowance = allowance
            .allowance
            .checked_sub(amount)
            .map_err(StdError::overflow)?;
        if let Some(exp) = expires {
            allowance.expires = exp;
        }
        ALLOWANCES.save(deps.storage, key, &allowance)?;
    } else {
        ALLOWANCES.remove(deps.storage, key);
    }

    let res = Response::new().add_attributes(vec![
        attr("action", "decrease_allowance"),
        attr("owner", info.sender),
        attr("owner_xyz_id", owner_xyz_id),
        attr("spender", spender),
        attr("amount", amount),
    ]);
    Ok(res)
}

// this can be used to update a lower allowance - call bucket.update with proper keys
pub fn deduct_allowance(
    storage: &mut dyn Storage,
    owner: &Addr,
    owner_xyz_id: String,
    spender: &Addr,
    block: &BlockInfo,
    amount: Uint128,
) -> Result<AllowanceResponse, ContractError> {
    ALLOWANCES.update(storage, (owner_xyz_id.as_str(), owner, spender), |current| {
        match current {
            Some(mut a) => {
                if a.expires.is_expired(block) {
                    Err(ContractError::Expired {})
                } else {
                    // deduct the allowance if enough
                    a.allowance = a
                        .allowance
                        .checked_sub(amount)
                        .map_err(StdError::overflow)?;
                    Ok(a)
                }
            }
            None => Err(ContractError::NoAllowance {}),
        }
    })
}

pub fn execute_burn_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
    owner_xyz_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let config = TOKEN_INFO.load(deps.storage)?;

    // Validate owner_xyz_id is owned by owner
    if !validate_nft_is_owned_by_wallet(
        &owner_xyz_id,
        &owner.to_string(),
        &deps.querier,
        &config.xyz_contract_address
    )? {
        return Err(ContractError::Unauthorized {});
    }

    // deduct allowance before doing anything else have enough allowance
    deduct_allowance(deps.storage, &owner_addr, owner_xyz_id.to_string(), &info.sender, &env.block, amount)?;

    // lower balance
    BALANCES.update(
        deps.storage,
        &owner_xyz_id.to_string(),
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(amount)?)
        },
    )?;
    // reduce total_supply
    TOKEN_INFO.update(deps.storage, |mut meta| -> StdResult<_> {
        meta.total_supply = meta.total_supply.checked_sub(amount)?;
        Ok(meta)
    })?;

    let res = Response::new().add_attributes(vec![
        attr("action", "burn_from"),
        attr("from", owner),
        attr("from_xyz_id", owner_xyz_id),
        attr("by", info.sender),
        attr("amount", amount),
    ]);
    Ok(res)
}

pub fn query_allowance(deps: Deps, owner: String, owner_xyz_id: String, spender: String) -> StdResult<AllowanceResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let spender_addr = deps.api.addr_validate(&spender)?;
    let allowance = ALLOWANCES
        .may_load(deps.storage, (&owner_xyz_id, &owner_addr, &spender_addr))?
        .unwrap_or_default();
    Ok(allowance)
}

#[cfg(test)]
mod tests {
    use super::*;

    use collectxyz_planet_metaverse::mock_querier::{DEFAULT_RAND, NFT_CONTRACT_ADDRESS, NFT_OWNER_ADDRESS, NOW, XYZ_NFT_ID, default_xyz_nft_data, mock_dependencies_custom};
    use collectxyz_experience::{TokenInfoResponse,XyzExperience, CollectXyzExperienceExecuteMsg};
    use cosmwasm_std::{Coin, Timestamp};
    use cosmwasm_std::testing::{mock_env, mock_info};

    use crate::contract::{execute, instantiate, query_balance, query_token_info};
    use crate::msg::{InstantiateMsg};

    fn coins(count: u64, token: &str) -> Vec<Coin> {
        let mut _coins = vec![];
        for _ in 1..count {
            _coins.push(
                Coin {
                    denom: token.to_string(),
                    amount: Uint128::from(1u64),
                }
            );
        }
        return _coins;
    }

    fn get_balance<T: Into<String>>(deps: Deps, address: T) -> Uint128 {
        query_balance(deps, address.into()).unwrap().balance
    }

    // this will set up the instantiation for other tests
    fn do_instantiate<T: Into<String>>(
        mut deps: DepsMut,
        _addr: T,
        amount: Uint128,
    ) -> TokenInfoResponse {
        let instantiate_msg = InstantiateMsg {
            name: "Auto Gen".to_string(),
            symbol: "AUTO".to_string(),
            decimals: 3,
            initial_balances: vec![XyzExperience {
                xyz_id: XYZ_NFT_ID.to_string(),
                amount
            }],
            mint: None,
            marketing: None,
            xyz_contract_address: Addr::unchecked(NFT_CONTRACT_ADDRESS),
        };
        let info = mock_info("creator", &[]);
        let env = mock_env();
        instantiate(deps.branch(), env, info, instantiate_msg).unwrap();
        query_token_info(deps.as_ref()).unwrap()
    }

    #[test]
    fn increase_decrease_allowances() {
        let mut deps = mock_dependencies_custom(
            Some(NFT_OWNER_ADDRESS.to_string()),
            Some(DEFAULT_RAND),
            Some(default_xyz_nft_data(NOW, true, None).clone()),
            vec![],
            &[],
        );

        let owner = NFT_OWNER_ADDRESS.to_string();
        let spender = String::from("addr0002");
        let info = mock_info(owner.as_ref(), &[]);
        let env = mock_env();
        do_instantiate(deps.as_mut(), owner.clone(), Uint128::new(12340000));

        // no allowance to start
        let allowance = query_allowance(
            deps.as_ref(),
            owner.clone(),
            XYZ_NFT_ID.to_string(),
            spender.clone(),

        ).unwrap();
        assert_eq!(allowance, AllowanceResponse::default());

        // set allowance with height expiration
        let allow1 = Uint128::new(7777);
        let expires = Expiration::AtHeight(5432);
        let msg = CollectXyzExperienceExecuteMsg::IncreaseAllowance {
            spender: spender.clone(),
            amount: allow1,
            expires: Some(expires),
            owner_xyz_id: XYZ_NFT_ID.to_string(),
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // ensure it looks good
        let allowance = query_allowance(
            deps.as_ref(),
            owner.clone(),
            XYZ_NFT_ID.to_string(),
            spender.clone()
        ).unwrap();
        assert_eq!(
            allowance,
            AllowanceResponse {
                allowance: allow1,
                expires
            }
        );

        // decrease it a bit with no expire set - stays the same
        let lower = Uint128::new(4444);
        let allow2 = allow1.checked_sub(lower).unwrap();
        let msg = CollectXyzExperienceExecuteMsg::DecreaseAllowance {
            spender: spender.clone(),
            amount: lower,
            expires: None,
            owner_xyz_id: XYZ_NFT_ID.to_string(),
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let allowance = query_allowance(
            deps.as_ref(),
            owner.clone(),
            XYZ_NFT_ID.to_string(),
            spender.clone()
        ).unwrap();
        assert_eq!(
            allowance,
            AllowanceResponse {
                allowance: allow2,
                expires
            }
        );

        // increase it some more and override the expires
        let raise = Uint128::new(87654);
        let allow3 = allow2 + raise;
        let new_expire = Expiration::AtTime(Timestamp::from_seconds(8888888888));
        let msg = CollectXyzExperienceExecuteMsg::IncreaseAllowance {
            spender: spender.clone(),
            amount: raise,
            expires: Some(new_expire),
            owner_xyz_id: XYZ_NFT_ID.to_string(),
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        let allowance = query_allowance(
            deps.as_ref(),
            owner.clone(),
            XYZ_NFT_ID.to_string(),
            spender.clone(),
        ).unwrap();
        assert_eq!(
            allowance,
            AllowanceResponse {
                allowance: allow3,
                expires: new_expire
            }
        );

        // decrease it below 0
        let msg = CollectXyzExperienceExecuteMsg::DecreaseAllowance {
            spender: spender.clone(),
            amount: Uint128::new(99988647623876347),
            expires: None,
            owner_xyz_id: XYZ_NFT_ID.to_string(),
        };
        execute(deps.as_mut(), env, info, msg).unwrap();
        let allowance = query_allowance(
            deps.as_ref(),
            owner,
            XYZ_NFT_ID.to_string(),
            spender
        ).unwrap();
        assert_eq!(allowance, AllowanceResponse::default());
    }

    #[test]
    fn allowances_independent() {
        let mut deps = mock_dependencies_custom(
            Some(NFT_OWNER_ADDRESS.to_string()),
            Some(DEFAULT_RAND),
            Some(default_xyz_nft_data(NOW, true, None)),
            vec![],
            &coins(2, "token"),
        );

        let owner = NFT_OWNER_ADDRESS.to_string();
        let owner_xyz_id = XYZ_NFT_ID.to_string();
        let spender = String::from("addr0002");
        let spender2 = String::from("addr0003");
        let info = mock_info(owner.as_ref(), &[]);
        let env = mock_env();
        do_instantiate(deps.as_mut(), &owner, Uint128::new(12340000));

        // no allowance to start
        assert_eq!(
            query_allowance(
                deps.as_ref(),
                owner.clone(),
                owner_xyz_id.to_string(),
                spender.clone()
            ).unwrap(),
            AllowanceResponse::default()
        );
        assert_eq!(
            query_allowance(
                deps.as_ref(),
                owner.clone(),
                owner_xyz_id.to_string(),
                spender2.clone()
            ).unwrap(),
            AllowanceResponse::default()
        );
        assert_eq!(
            query_allowance(
                deps.as_ref(),
                spender.clone(),
                owner_xyz_id.to_string(),
                spender2.clone()
            ).unwrap(),
            AllowanceResponse::default()
        );

        // set allowance with height expiration
        let allow1 = Uint128::new(7777);
        let expires = Expiration::AtHeight(5432);
        let msg = CollectXyzExperienceExecuteMsg::IncreaseAllowance {
            spender: spender.clone(),
            amount: allow1,
            expires: Some(expires),
            owner_xyz_id: owner_xyz_id.to_string()
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // set other allowance with no expiration
        let allow2 = Uint128::new(87654);
        let msg = CollectXyzExperienceExecuteMsg::IncreaseAllowance {
            spender: spender2.clone(),
            amount: allow2,
            expires: None,
            owner_xyz_id: owner_xyz_id.to_string(),
        };
        execute(deps.as_mut(), env, info, msg).unwrap();

        // check they are proper
        let expect_one = AllowanceResponse {
            allowance: allow1,
            expires,
        };
        let expect_two = AllowanceResponse {
            allowance: allow2,
            expires: Expiration::Never {},
        };
        assert_eq!(
            query_allowance(
                deps.as_ref(),
                owner.clone(),
                owner_xyz_id.to_string(),
                spender.clone()
            ).unwrap(),
            expect_one
        );
        assert_eq!(
            query_allowance(
                deps.as_ref(),
                owner.clone(),
                owner_xyz_id.to_string(),
                spender2.clone()
            ).unwrap(),
            expect_two
        );
        assert_eq!(
            query_allowance(
                deps.as_ref(),
                spender.clone(),
                owner_xyz_id.to_string(),
                spender2.clone()
            ).unwrap(),
            AllowanceResponse::default()
        );
    }

    #[test]
    fn no_self_allowance() {
        let mut deps = mock_dependencies_custom(
            Some(NFT_OWNER_ADDRESS.to_string()),
            Some(DEFAULT_RAND),
            Some(default_xyz_nft_data(NOW, false, None)),
            vec![],
            &coins(2, "token")
        );

        let owner = NFT_OWNER_ADDRESS.to_string();
        let owner_xyz_id = XYZ_NFT_ID.to_string();
        let info = mock_info(owner.as_ref(), &[]);
        let env = mock_env();
        do_instantiate(deps.as_mut(), &owner, Uint128::new(12340000));

        // self-allowance
        let msg = CollectXyzExperienceExecuteMsg::IncreaseAllowance {
            spender: owner.clone(),
            amount: Uint128::new(7777),
            expires: None,
            owner_xyz_id: owner_xyz_id.to_string()
        };
        let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
        assert_eq!(err, ContractError::CannotSetOwnAccount {});

        // decrease self-allowance
        let msg = CollectXyzExperienceExecuteMsg::DecreaseAllowance {
            spender: owner,
            amount: Uint128::new(7777),
            expires: None,
            owner_xyz_id: owner_xyz_id.to_string()
        };
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert_eq!(err, ContractError::CannotSetOwnAccount {});
    }

    #[test]
    fn burn_from_respects_limits() {
        let mut deps = mock_dependencies_custom(
            Some(NFT_OWNER_ADDRESS.to_string()),
            Some(DEFAULT_RAND),
            Some(default_xyz_nft_data(NOW, false, None)),
            vec![],
            &[],
        );
        let owner = NFT_OWNER_ADDRESS.to_string();
        let owner_xyz_id = XYZ_NFT_ID.to_string();
        let spender = String::from("addr0002");

        let start = Uint128::new(999999);
        do_instantiate(deps.as_mut(), &owner, start);

        // provide an allowance
        let allow1 = Uint128::new(77777);
        let msg = CollectXyzExperienceExecuteMsg::IncreaseAllowance {
            spender: spender.clone(),
            amount: allow1,
            expires: None,
            owner_xyz_id: owner_xyz_id.to_string(),
        };
        let info = mock_info(owner.as_ref(), &[]);
        let env = mock_env();
        execute(deps.as_mut(), env, info, msg).unwrap();

        // valid burn of part of the allowance
        let transfer = Uint128::new(44444);
        let msg = CollectXyzExperienceExecuteMsg::BurnFrom {
            owner: owner.clone(),
            amount: transfer,
            owner_xyz_id: owner_xyz_id.to_string(),
        };
        let info = mock_info(spender.as_ref(), &[]);
        let env = mock_env();
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes[0], attr("action", "burn_from"));

        // make sure money burnt
        assert_eq!(
            get_balance(deps.as_ref(), owner_xyz_id.to_string()),
            start.checked_sub(transfer).unwrap()
        );

        // ensure it looks good
        let allowance = query_allowance(deps.as_ref(), owner.clone(), owner_xyz_id.to_string(), spender.clone()).unwrap();
        let expect = AllowanceResponse {
            allowance: allow1.checked_sub(transfer).unwrap(),
            expires: Expiration::Never {},
        };
        assert_eq!(expect, allowance);

        // cannot burn more than the allowance
        let msg = CollectXyzExperienceExecuteMsg::BurnFrom {
            owner: owner.clone(),
            amount: Uint128::new(33443),
            owner_xyz_id: owner_xyz_id.to_string(),
        };
        let info = mock_info(spender.as_ref(), &[]);
        let env = mock_env();
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert!(matches!(err, ContractError::Std(StdError::Overflow { .. })));

        // let us increase limit, but set the expiration (default env height is 12_345)
        let info = mock_info(owner.as_ref(), &[]);
        let env = mock_env();
        let msg = CollectXyzExperienceExecuteMsg::IncreaseAllowance {
            spender: spender.clone(),
            amount: Uint128::new(1000),
            expires: Some(Expiration::AtHeight(env.block.height)),
            owner_xyz_id: owner_xyz_id.to_string(),
        };
        execute(deps.as_mut(), env, info, msg).unwrap();

        // we should now get the expiration error
        let msg = CollectXyzExperienceExecuteMsg::BurnFrom {
            owner,
            amount: Uint128::new(33443),
            owner_xyz_id: owner_xyz_id.to_string(),
        };
        let info = mock_info(spender.as_ref(), &[]);
        let env = mock_env();
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert_eq!(err, ContractError::Expired {});
    }

    #[test]
    fn test_non_owner_cant_increase_allowance() {
        let mut deps = mock_dependencies_custom(
            Some(NFT_OWNER_ADDRESS.to_string()),
            Some(DEFAULT_RAND),
            Some(default_xyz_nft_data(NOW, true, None).clone()),
            vec![],
            &[],
        );

        let owner = "NonNFTOwner".to_string();
        let spender = String::from("addr0002");
        let info = mock_info(owner.as_ref(), &[]);
        let env = mock_env();
        do_instantiate(deps.as_mut(), owner.clone(), Uint128::new(12340000));

        // no allowance to start
        let allowance = query_allowance(
            deps.as_ref(),
            owner.clone(),
            XYZ_NFT_ID.to_string(),
            spender.clone(),

        ).unwrap();
        assert_eq!(allowance, AllowanceResponse::default());

        // set allowance with height expiration
        let allow1 = Uint128::new(7777);
        let expires = Expiration::AtHeight(5432);
        let msg = CollectXyzExperienceExecuteMsg::IncreaseAllowance {
            spender: spender.clone(),
            amount: allow1,
            expires: Some(expires),
            owner_xyz_id: XYZ_NFT_ID.to_string(),
        };
        let result = execute(deps.as_mut(), env.clone(), info.clone(), msg);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err(), ContractError::Unauthorized {});
    }

    #[test]
    fn test_non_owner_cant_decrease_allowance() {
        let mut deps = mock_dependencies_custom(
            Some(NFT_OWNER_ADDRESS.to_string()),
            Some(DEFAULT_RAND),
            Some(default_xyz_nft_data(NOW, true, None).clone()),
            vec![],
            &[],
        );

        let owner = "NonNFTOwner".to_string();
        let spender = String::from("addr0002");
        let info = mock_info(owner.as_ref(), &[]);
        let env = mock_env();
        do_instantiate(deps.as_mut(), owner.clone(), Uint128::new(12340000));

        // no allowance to start
        let allowance = query_allowance(
            deps.as_ref(),
            owner.clone(),
            XYZ_NFT_ID.to_string(),
            spender.clone(),

        ).unwrap();
        assert_eq!(allowance, AllowanceResponse::default());

        // set allowance with height expiration
        let allow1 = Uint128::new(7777);
        let expires = Expiration::AtHeight(5432);
        let msg = CollectXyzExperienceExecuteMsg::DecreaseAllowance {
            spender: spender.clone(),
            amount: allow1,
            expires: Some(expires),
            owner_xyz_id: XYZ_NFT_ID.to_string(),
        };
        let result = execute(deps.as_mut(), env.clone(), info.clone(), msg);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err(), ContractError::Unauthorized {});
    }
}