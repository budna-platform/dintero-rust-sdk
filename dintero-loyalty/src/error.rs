use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoyaltyError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },
    
    #[error("Validation error: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, LoyaltyError>;
