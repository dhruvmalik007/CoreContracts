use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Pausable-base: paused")]
    Paused,

    #[error("Pausable-base: not paused")]
    NotPaused,
}
