use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Index: {0} out of bound for list {1}")]
    IndexOutOfBound(String, String),

    #[error("Invalid Resource Id: {0}")]
    InvalidResourceId(String),

    #[error("Invalid Objective Id: {0}")]
    InvalidObjectiveId(String),

    #[error("Storage Conflict: {0}")]
    StorageConflict(String),

    #[error("Objective has not been reveald yet")]
    ObjectiveNotStarted {},

    #[error("Objective {0} was not found")]
    ObjectiveNotFound(String),

    #[error("Attempting to complete objective with no goal.")]
    NoGoalSetForObjective {},

    #[error("Unable to complete quest.")]
    UnableToCompleteQuest {},

    #[error("Unable to complete quest, no funds.")]
    UnableToCompleteQuestNoFunds {},

    #[error("Unable to complete quest since it is still active.")]
    UnableToCompleteActiveQuest {},

    #[error("Unable to complete the same quest more than once.")]
    UnableToCompleteQuestAgain {},

    #[error("Not able to claim Quest winnings yet.")]
    TemporarilyUnableToCompleteQuest {},

    #[error("Objective Already completed.")]
    ObjectiveAlreadyCompleted {},

    #[error("Can't modify an Objective that has already started.")]
    ObjectiveAlreadyStarted {},

    #[error("No UST funds deposited.")]
    FailedToDepositPrizePool {},

    #[error("Quest has expired.")]
    ExpiredQuest {},

    #[error("Config field: {0} value is in valid. Reason: {1}")]
    InvalidConfig(String, String),
}
