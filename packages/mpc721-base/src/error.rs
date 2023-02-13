use thiserror::Error;

/// ## Description
/// This enum describes mpc721 contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Token id is above max mint")]
    MintLimitExceeded,
    #[error("Token with specified id is already minted")]
    Minted,

    #[error("Not found")]
    NotFound,

    #[error("Incorrect Owner")]
    IncorrectOwner,
}
