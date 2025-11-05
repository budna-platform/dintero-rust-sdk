use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("API error ({code}): {message}")]
    Api { code: String, message: String },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Rate limited, retry after {retry_after:?}")]
    RateLimited { retry_after: Option<Duration> },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, Error>;
