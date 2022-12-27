export interface QuestConfig {
    quest_name: string
    start_time: string
    quest_duration_seconds: string
    xp_contract: string
    objectives: Objective[]
    xyz_nft_contract: string
    randomness_contract: string
    resource_configs: ResourceConfig,
}

export interface Coin {
    denom: string
    amount: string
}

export interface Objective {
    objective_id: string
    objective_start_time: string
    duration: string
    multiplier: string
    late_penalty: string
    goal: Goal
    desc: string
    possible_goals_info: GoalInfo[]
}

export interface CompleteObjective {
    xyz_id: string
    completed_timestamp: string
    objective: Objective
}
  
export interface GoalInfo {
    name: string,
    xp_amount: string
    rock_weighting: string
    ice_weighting: string
    metal_weighting: string
    gas_weighting: string
    water_weighting: string
    gem_weighting: string
    life_weighting: string
}

export interface Goal {
    name: string
    xp_reward: string
    required_resources: RequiredResource[]
}

export interface RequiredResource {
    resource_id: string
    required_amount: string
}

export interface ResourceConfig {
    rock_contract: string
    ice_contract: string
    metal_contract: string
    gas_contract: string
    water_contract: string
    gem_contract: string
    life_contract: string
}