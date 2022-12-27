use collectxyz_planet_metaverse::util::validate_nft_is_owned_by_wallet;
use cosmwasm_std::{BankMsg, Coin, DepsMut, Env, MessageInfo, Response, BlockInfo};

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::{Completed, CONFIG, OWNER, ALLOW_PRIZE_CLAIM, Objective, GoalInfo, Config};
use crate::util::{calculate_xyz_reward, get_random_for_objective_start_time};

pub fn execute_instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    OWNER.save(deps.storage, &info.sender)?;
    msg.config.is_valid(&env.block)?;
    ALLOW_PRIZE_CLAIM.save(deps.storage, &false)?;
    CONFIG.save(deps.storage, &msg.config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

pub fn execute_complete_objective(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    xyz_id: String,
    objective_id: u32,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let owner = &info.sender.to_string();

    // Fail if user does not own xyz
    if !validate_nft_is_owned_by_wallet(&xyz_id, owner, &deps.querier, &config.xyz_nft_contract)? {
        return Err(ContractError::Unauthorized {});
    }

    // Fail if Quest is expired
    if config.is_quest_completed(&env.block) {
        return Err(ContractError::ExpiredQuest {});
    }

    // Fail if no objective exists
    let mut objective = match config.objectives.get(objective_id as usize) {
        Some(obj) => Ok(obj.clone()),
        None => Err(ContractError::ObjectiveNotFound(objective_id.to_string())),
    }?;

    // Fail if objective is not yet started
    if !objective.is_started(&env.block) {
        return Err(ContractError::ObjectiveNotStarted {});
    }

    // Get the objective for the objective's timestamp
    let random_nums = get_random_for_objective_start_time(
        &deps.querier,
        &config.randomness_contract,
        objective_id,
        &objective,
    )?;
    let objective = config.get_objective(&env.block, &random_nums, &mut objective)?;

    // Fail if user does not have enough resources to complete the objective
    // if these messages fail, we will fail to be able to complete this objective for the user
    let messages = objective.attempt_to_complete(config, owner, &xyz_id)?;

    // Save completed objective for xyz_id.
    // This will only save if the transaction succeeds.
    // The transaction will only succeed if the user has enough resources to burn
    let completed = Completed::default();
    let (complete, _) =
        completed.save_completed_objective(deps.storage, env.block.time, &xyz_id, &objective)?;

    // Burn resources and record completed objective for xyz_id
    return Ok(Response::default()
        .add_messages(messages)
        .add_attribute("method", "execute")
        .add_attribute("action", "complete_objective")
        .add_attribute("objective", serde_json::to_string(&complete).unwrap()));
}

pub fn execute_complete_quest(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    xyz_id: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let owner = &info.sender.to_string();
    let is_prize_claim_allowed = ALLOW_PRIZE_CLAIM.load(deps.storage)?;

    if !is_prize_claim_allowed {
        return Err(ContractError::TemporarilyUnableToCompleteQuest {});
    }

    // Fail if user does not own xyz
    if !validate_nft_is_owned_by_wallet(&xyz_id, owner, &deps.querier, &config.xyz_nft_contract)? {
        return Err(ContractError::Unauthorized {});
    }

    /*
    Dont allow complete quest until the quest is expired

    This ensures that the reward allocation is done after everyone
    who can has already completed their objectives.
    */
    if !config.is_quest_completed(&env.block) {
        return Err(ContractError::UnableToCompleteActiveQuest {});
    }

    let completed = Completed::default();
    if completed.is_quest_already_completed_for(deps.storage, &xyz_id) {
        return Err(ContractError::UnableToCompleteQuestAgain {});
    }

    let reward: Coin = calculate_xyz_reward(&xyz_id, &env, &deps.as_ref())?;

    let send_reward_message = BankMsg::Send {
        to_address: owner.to_string(),
        amount: vec![reward.clone()],
    };

    completed.mark_completed_quest_for(&env.block, deps.storage, &xyz_id)?;

    return Ok(Response::default()
        .add_attribute("method", "execute")
        .add_attribute("action", "complete_quest")
        .add_attribute("reward", serde_json::to_string(&reward).unwrap())
        .add_message(send_reward_message));
}

pub fn execute_prize_pool_deposit(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    let is_prize_claim_allowed = ALLOW_PRIZE_CLAIM.load(deps.storage)?;

    if is_prize_claim_allowed {
        return Err(ContractError::FailedToDepositPrizePool {});
    }

    if owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let funds = info.funds.iter().find(|fund| fund.denom == "uusd");

    if funds.is_none() {
        return Err(ContractError::FailedToDepositPrizePool {});
    }

    Completed::default().update_reward_funds_deposited(
        deps.storage, funds.unwrap().clone()
    )?;

    return Ok(Response::default()
        .add_attribute("method", "execute")
        .add_attribute("action", "prize_pool_deposit")
        .add_attribute("reward", serde_json::to_string(funds.unwrap()).unwrap()));
}

pub fn execute_set_allow_quest_completion(
    deps: DepsMut,
    info: MessageInfo,
    allow_prize_claim: bool,
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;

    if owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    ALLOW_PRIZE_CLAIM.save(deps.storage, &allow_prize_claim)?;
    return Ok(Response::default()
        .add_attribute("method", "execute")
        .add_attribute("action", "set_allow_quest_completion")); 
}

pub fn execute_update_objective(
    deps: DepsMut,
    info: MessageInfo,
    block: BlockInfo,
    objective_id: u32,
    possible_goal_info: Vec<GoalInfo>
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;

    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    let mut config: Config = CONFIG.load(deps.storage)?;
    let objective_option: Option<&mut Objective> = config.objectives.get_mut(objective_id as usize);

    if objective_option.is_none() {
        return Err(ContractError::ObjectiveNotFound(objective_id.to_string()));
    }

    let objective = objective_option.unwrap();
    if objective.is_started(&block) {
        return Err(ContractError::ObjectiveAlreadyStarted {});
    }

    objective.possible_goals_info = possible_goal_info;

    CONFIG.save(deps.storage, &config)?;

    return Ok(Response::default()
        .add_attribute("method", "execute")
        .add_attribute("action", "update_objective")); 
}
