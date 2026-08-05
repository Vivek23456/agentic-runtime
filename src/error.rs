use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("invalid state transition")]
    InvalidStateTransition,

    #[error("task not found")]
    TasknotFound,
}
