use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{Config, GoalInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub config: Config,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Called by user to complete an objective given the ID.
	// This will only pass
	//  1. xyz_id is owned by sender
	//  2. xyz_id contains the required resources
	//  3. objective has started
	//  4. quest has started
	//  5. quest has not ended
	// the resources will be burned on success.
    CompleteObjective {
        xyz_id: String,
        objective_id: u32
    },

	PrizePoolDeposit {},

	AllowQuestClaims {
		allow_claims: bool
	},
    /// Called by user to complete the quest
	// This will only pass
	//  1. xyz_id has completed all objectives
	//  2. quest is not expired
	//	3. quest has started
	// the resources will be burned on success.
	CompleteQuest {
		xyz_id: String
	},

	UpdateObjective {
		objective_id: u32,
		possible_goal_info: Vec<GoalInfo>
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCompleted { xyz_id: String },
    GetObjectives {},
    CurrentConfig {},
	GetIsQuestCompleted { xyz_id: String },
    GetReward { xyz_id: String },
	GetObjectiveCompletedCount { objective_id: u32 }
}
