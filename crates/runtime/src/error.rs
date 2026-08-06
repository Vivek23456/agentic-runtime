use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("invalid state transition")]
    InvalidStateTransition,

    #[error("task not found")]
    TaskNotFound,

    #[error("{0}")]
    Internal(String),
}