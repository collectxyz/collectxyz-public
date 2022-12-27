use collectxyz_planet_metaverse::{experience::XyzExperienceMintInfo, util::burn_resource};
use cosmwasm_std::{
    Addr, BlockInfo, Coin, StdResult, Storage, Timestamp, Uint128, Uint64, WasmMsg,
};
use cw_storage_plus::{Item, Map};
use fixed::types::U64F64;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ContractError;

pub const OWNER: Item<Addr> = Item::new("owner");

/// Configs
pub const CONFIG: Item<Config> = Item::new("config");
pub const ALLOW_PRIZE_CLAIM: Item<bool> = Item::new("allow_prize_claim");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub quest_name: String,
    pub start_time: Timestamp,
    pub quest_duration_seconds: Uint64,
    pub xp_contract: Addr,
    pub objectives: Vec<Objective>,
    pub xyz_nft_contract: Addr,
    pub randomness_contract: Addr,
    pub resource_configs: ResourceConfig,
}

impl Config {
    pub fn is_valid(&self, block: &BlockInfo) -> Result<(), ContractError> {
        if self.start_time <= block.time {
            return Err(ContractError::InvalidConfig(
                "start_time".to_string(),
                "start_time is in the past".to_string(),
            ));
        }

        let quest_endtime = self
            .start_time
            .plus_seconds(self.quest_duration_seconds.into());
        for (expected_id, objective) in self.objectives.clone().iter().enumerate() {
            if objective.objective_start_time < self.start_time {
                return Err(ContractError::InvalidConfig(
                    "objectives".to_string(),
                    "objective_start_time must be after Quest start_time".to_string(),
                ));
            }
            if objective.objective_start_time > quest_endtime {
                return Err(ContractError::InvalidConfig(
                    "objectives".to_string(),
                    "objective_start_time must be before Quest end_time".to_string(),
                ));
            }
            if objective.completes() > quest_endtime {
                return Err(ContractError::InvalidConfig(
                    "objectives".to_string(),
                    "objective completion must be before Quest end_time".to_string(),
                ));
            }
            if objective.objective_id != expected_id as u32 {
                return Err(ContractError::InvalidConfig(
                    "objectives".to_string(),
                    format!(
                        "objective_id: got {}, but expected {}",
                        objective.objective_id, expected_id
                    ),
                ));
            }
        }

        return Ok(());
    }

    pub fn is_quest_completed(&self, block: &BlockInfo) -> bool {
        return self
            .start_time
            .plus_seconds(self.quest_duration_seconds.into())
            < block.time;
    }

    pub fn get_objective(
        &self,
        block: &BlockInfo,
        random_numbers: &Vec<u8>,
        objective: &mut Objective,
    ) -> Result<Objective, ContractError> {
        let late_penalty = if objective.completes() < block.time {
            objective.late_penalty
        } else {
            0
        };
        if !objective.possible_goals_info.is_empty() {
            let position = usize::from(random_numbers[0]) % objective.possible_goals_info.len();
            let goal = match objective.possible_goals_info.get(position) {
                Some(possible_goal_info) => {
                    possible_goal_info.build_goal(objective.multiplier, late_penalty)
                }
                None => {
                    return Err(ContractError::IndexOutOfBound(
                        position.to_string(),
                        "possible_goal".to_string(),
                    ))
                }
            };
            objective.goal = Some(goal);
        }
        return Ok(objective.clone());
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ResourceConfig {
    pub rock_contract: Addr,
    pub ice_contract: Addr,
    pub metal_contract: Addr,
    pub gas_contract: Addr,
    pub water_contract: Addr,
    pub gem_contract: Addr,
    pub life_contract: Addr,
}

impl ResourceConfig {
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
}

/// The blue print for generating a goal
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GoalInfo {
    pub name: String,
    pub xp_amount: Uint128,
    pub rock_weighting: Option<Uint64>,
    pub ice_weighting: Option<Uint64>,
    pub metal_weighting: Option<Uint64>,
    pub gas_weighting: Option<Uint64>,
    pub water_weighting: Option<Uint64>,
    pub gem_weighting: Option<Uint64>,
    pub life_weighting: Option<Uint64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Goal {
    pub name: String,
    pub xp_reward: Uint128,
    pub required_resources: Vec<RequiredResource>,
}

impl GoalInfo {
    fn amount(weight: Uint64, multiplier: u32) -> Uint128 {
        return (U64F64::from_num(u64::from(weight)) / 100u128 * u128::from(multiplier))
            .ceil()
            .to_num::<u128>()
            .into();
    }

    fn build_goal(&self, multiplier: u32, late_penalty: u32) -> Goal {
        let xp_reward = self.xp_amount;

        let multiplier = multiplier + late_penalty;
        let mut required_resources: Vec<RequiredResource> = vec![];
        if self.rock_weighting.is_some() {
            required_resources.push(RequiredResource {
                resource_id: "xyzROCK".into(),
                required_amount: GoalInfo::amount(self.rock_weighting.unwrap(), multiplier),
            })
        }

        if self.ice_weighting.is_some() {
            required_resources.push(RequiredResource {
                resource_id: "xyzICE".into(),
                required_amount: GoalInfo::amount(self.ice_weighting.unwrap(), multiplier),
            })
        }

        if self.metal_weighting.is_some() {
            required_resources.push(RequiredResource {
                resource_id: "xyzMETAL".into(),
                required_amount: GoalInfo::amount(self.metal_weighting.unwrap(), multiplier),
            })
        }

        if self.gas_weighting.is_some() {
            required_resources.push(RequiredResource {
                resource_id: "xyzGAS".into(),
                required_amount: GoalInfo::amount(self.gas_weighting.unwrap(), multiplier),
            })
        }

        if self.water_weighting.is_some() {
            required_resources.push(RequiredResource {
                resource_id: "xyzWATER".into(),
                required_amount: GoalInfo::amount(self.water_weighting.unwrap(), multiplier),
            })
        }

        if self.gem_weighting.is_some() {
            required_resources.push(RequiredResource {
                resource_id: "xyzGEM".into(),
                required_amount: GoalInfo::amount(self.gem_weighting.unwrap(), multiplier),
            })
        }

        if self.life_weighting.is_some() {
            required_resources.push(RequiredResource {
                resource_id: "xyzLIFE".into(),
                required_amount: GoalInfo::amount(self.life_weighting.unwrap(), multiplier),
            })
        }

        return Goal {
            name: self.name.to_string(),
            xp_reward,
            required_resources,
        };
    }
}

/// Generated objective which is part of the quest
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Objective {
    pub objective_id: u32,
    pub objective_start_time: Timestamp,
    pub duration: u64,
    pub multiplier: u32,
    pub late_penalty: u32,
    pub goal: Option<Goal>,
    pub possible_goals_info: Vec<GoalInfo>,
    pub desc: Option<String>,
}

impl Objective {
    pub fn is_started(&self, block: &BlockInfo) -> bool {
        return block.time > self.objective_start_time;
    }

    pub fn completes(&self) -> Timestamp {
        return self.objective_start_time.plus_seconds(self.duration);
    }

    pub fn attempt_to_complete(
        &self,
        config: Config,
        owner: &String,
        owner_xyz_id: &String,
    ) -> Result<Vec<WasmMsg>, ContractError> {
        if self.goal.is_none() {
            return Err(ContractError::ObjectiveNotStarted {});
        }

        let goal = self.goal.clone().unwrap();
        let mut messages = vec![];
        for required_resource in goal.required_resources {
            let message = burn_resource(
                owner.to_string(),
                owner_xyz_id.to_string(),
                config
                    .resource_configs
                    .resource_addr(&required_resource.resource_id)?
                    .to_string(),
                required_resource
                    .required_amount
                    .checked_mul(1000000u128.into())
                    .unwrap_or(Uint128::MAX),
            )?;
            messages.push(message);
        }

        messages.push(
            XyzExperienceMintInfo {
                experience_contract_address: config.xp_contract,
                complete_task_experience_amount: goal.xp_reward,
            }
            .mint_experince(owner_xyz_id.to_string())?,
        );
        return Ok(messages);
    }
}

/// Generated objective which is part of the quest
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CompleteObjective {
    pub xyz_id: String,
    pub completed_timestamp: Timestamp,
    pub objective: Objective,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CompleteQuest {
    pub xyz_id: String,
    pub completed_timestamp: Timestamp,
}

impl CompleteObjective {
    pub fn is_completed_late(&self) -> bool {
        return self.objective.completes() < self.completed_timestamp;
    }
}

pub struct Completed<'a> {
    completed_objectives_by_xyz_id: Map<'a, String, Vec<CompleteObjective>>,
    completed_objective_count: Map<'a, String, Uint128>,
    completed_quest_by_xyz_id: Map<'a, String, CompleteQuest>,
    reward_pool_funds: Item<'a, Coin>,
}

impl<'a> Completed<'a> {
    pub fn new(
        completed_objective_key: &'a str,
        completed_quest_key: &'a str,
        reward_pool_funds_key: &'a str,
        completed_objective_count_key: &'a str,
    ) -> Self {
        Completed {
            completed_objectives_by_xyz_id: Map::new(completed_objective_key),
            completed_objective_count: Map::new(completed_objective_count_key),
            completed_quest_by_xyz_id: Map::new(completed_quest_key),
            reward_pool_funds: Item::new(reward_pool_funds_key),
        }
    }

    pub fn save_completed_objective(
        &self,
        storage: &mut dyn Storage,
        completed_timestamp: Timestamp,
        xyz_id: &String,
        objective: &Objective,
    ) -> Result<(CompleteObjective, Vec<CompleteObjective>), ContractError> {
        let completed = CompleteObjective {
            xyz_id: xyz_id.to_string(),
            completed_timestamp,
            objective: objective.clone(),
        };
        if objective.goal.is_none() {
            return Err(ContractError::NoGoalSetForObjective {});
        }

        let mut completed_so_far = self.get_completed_for(&xyz_id, storage)?;

        if completed_so_far
            .iter()
            .any(|complete| complete.objective == objective.clone())
        {
            return Err(ContractError::ObjectiveAlreadyCompleted {});
        }

        let objective_count_key = objective.objective_id.to_string();
        self.completed_objective_count.update(
            storage,
            objective_count_key,
         |count: Option<Uint128>| -> StdResult<_> {
                return if let Some(c) = count {
                    Ok(c.checked_add(1u128.into()).unwrap_or(Uint128::MAX))
                } else {
                    Ok(Uint128::from(1u128))
                }
            }
        )?;

        completed_so_far.push(completed.clone());
        match self.completed_objectives_by_xyz_id.save(
            storage,
            xyz_id.to_string(),
            &completed_so_far,
        ) {
            Ok(_) => Ok((completed, completed_so_far)),
            Err(err) => Err(ContractError::StorageConflict(err.to_string())),
        }
    }

    pub fn get_completed_for(
        &self,
        xyz_id: &String,
        storage: &dyn Storage,
    ) -> Result<Vec<CompleteObjective>, ContractError> {
        let completed = self
            .completed_objectives_by_xyz_id
            .load(storage, xyz_id.to_string())
            .unwrap_or(vec![]);
        Ok(completed)
    }

    pub fn total_quest_completed_count(
        &self,
        storage: &dyn Storage,
        config: &Config,
    ) -> Result<usize, ContractError> {
        let count: usize = self
            .completed_objectives_by_xyz_id
            .range(storage, None, None, cosmwasm_std::Order::Ascending)
            .map(|entry| Ok(entry?.clone()))
            .collect::<StdResult<Vec<(Vec<u8>, Vec<CompleteObjective>)>>>()?
            .iter()
            .filter(|entry| entry.1.len() == config.objectives.len())
            .count();
        return Ok(count);
    }

    pub fn mark_completed_quest_for(
        &self,
        block: &BlockInfo,
        storage: &mut dyn Storage,
        xyz_id: &String,
    ) -> Result<CompleteQuest, ContractError> {
        let completed_quest = CompleteQuest {
            xyz_id: xyz_id.to_string(),
            completed_timestamp: block.time,
        };
        self.completed_quest_by_xyz_id
            .save(storage, xyz_id.to_string(), &completed_quest)?;
        return Ok(completed_quest);
    }

    pub fn is_quest_already_completed_for(&self, storage: &dyn Storage, xyz_id: &String) -> bool {
        self.completed_quest_by_xyz_id
            .has(storage, xyz_id.to_string())
    }

    pub fn update_reward_funds_deposited(
        &self,
        storage: &mut dyn Storage,
        funds: Coin,
    ) -> Result<(), ContractError> {
        match self.reward_pool_funds.load(storage) {
            Ok(current_reward) => {
                let updated_reward_amount = current_reward
                    .amount
                    .checked_add(funds.amount)
                    .map_err(|_| ContractError::FailedToDepositPrizePool {})?;
                let new_funds = Coin::new(updated_reward_amount.u128(), funds.denom);
                self.reward_pool_funds.save(storage, &new_funds)?;
            }
            Err(_) => {
                self.reward_pool_funds.save(storage, &funds)?;
            }
        }

        Ok(())
    }

    pub fn get_deposited_reward_funds(&self, storage: &dyn Storage) -> Result<Coin, ContractError> {
        let reward_funds = self.reward_pool_funds.load(storage)?;
        Ok(reward_funds)
    }

    pub fn total_completed_for_objective(
        &self,
        storage: &dyn Storage,
        objective_id: &u32,
    ) -> Result<Uint128, ContractError> {
        return Ok(self.completed_objective_count.load(storage, objective_id.to_string()).unwrap_or(0u128.into()));
    }
}

impl Default for Completed<'static> {
    fn default() -> Self {
        Self::new(
            "completed_objective",
            "completed_objective_counts",
            "completed_quest",
            "reward_pool_funds",
        )
    }
}

/// Represents the required number of resources
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RequiredResource {
    pub resource_id: String,
    pub required_amount: Uint128,
}
