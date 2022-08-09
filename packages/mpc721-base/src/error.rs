use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Token with specified id is already minted")]
    Minted,

    #[error("Not found")]
    NotFound,
}
