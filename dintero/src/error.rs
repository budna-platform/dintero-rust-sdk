//! Error types for the Dintero SDK.

use std::time::Duration;

/// Error types that can occur when using the Dintero SDK.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Authentication failed.
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// API returned an error.
    #[error("API error ({code}): {message}")]
    Api { code: String, message: String },

    /// Validation error for input parameters.
    #[error("Validation error: {0}")]
    Validation(String),

    /// Rate limit exceeded.
    #[error("Rate limited, retry after {retry_after:?}")]
    RateLimited { retry_after: Option<Duration> },

    /// Serialization/deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Invalid URL.
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Result type for Dintero SDK operations.
pub type Result<T> = std::result::Result<T, Error>;
