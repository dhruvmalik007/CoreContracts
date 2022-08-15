use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Ownable-base: caller is not the owner")]
    CallerIsNotTheOwner,
}
