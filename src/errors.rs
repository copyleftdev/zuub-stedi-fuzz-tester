use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Internal server error")]
    InternalServerError,

    #[error("Rate limit exceeded")]
    RateLimitLimited,

    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}
