use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not found")]
    NotFound,
}
