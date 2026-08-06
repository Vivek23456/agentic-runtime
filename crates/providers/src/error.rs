use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("authentication failed")]
    Authentication,

    #[error("request timed out")]
    Timeout,

    #[error("rate limited")]
    RateLimited,

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("provider error: {0}")]
    Internal(String),
}