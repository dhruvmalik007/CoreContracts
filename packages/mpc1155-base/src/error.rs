use thiserror::Error;

/// ## Description
/// This enum describes mpc1155 contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not found")]
    NotFound,
}
