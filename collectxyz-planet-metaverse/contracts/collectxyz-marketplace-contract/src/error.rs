use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Storage Conflict")]
    StorageConflict {},

    #[error("Invalid Listing Price")]
    InvalidListingPrice {},

    #[error("Invalid Resource Id: {0}")]
    InvalidResourceId(String),

    #[error("Partial Resource Amount: {0}")]
    PartialResourceAmount(Uint128),

    #[error("Duplicate Resource Id: {0}")]
    DuplicateResourceId(String),

    #[error("Empty Resource Bundle")]
    EmptyResourceBundle {},

    #[error("Inactive Listing")]
    InactiveListing {},

    #[error("Can't Take Own Listing")]
    CantTakeOwnListing {},
}
