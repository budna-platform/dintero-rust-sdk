//! Secret management for checkout sessions.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureSecret {
    pub id: String,
    pub secret: String,
    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateSignatureSecretRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateSignatureSecretRequest {
    pub fn new() -> Self {
        Self { description: None }
    }

    pub fn with_description(description: impl Into<String>) -> Self {
        Self { description: Some(description.into()) }
    }
}

impl Default for CreateSignatureSecretRequest {
    fn default() -> Self {
        Self::new()
    }
}
