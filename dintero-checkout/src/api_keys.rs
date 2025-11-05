//! API key management for the Checkout API.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateApiKeyRequest {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    pub id: String,
    pub name: String,
    pub key: String,
    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateApiKeyResponse {
    pub id: String,
    pub name: String,
    pub key: String,
    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl CreateApiKeyRequest {
    pub fn builder() -> CreateApiKeyRequestBuilder {
        CreateApiKeyRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreateApiKeyRequestBuilder {
    name: Option<String>,
    expires_at: Option<String>,
}

impl CreateApiKeyRequestBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn expires_at(mut self, expires_at: impl Into<String>) -> Self {
        self.expires_at = Some(expires_at.into());
        self
    }

    pub fn build(self) -> Result<CreateApiKeyRequest, String> {
        Ok(CreateApiKeyRequest {
            name: self.name.ok_or("name is required")?,
            expires_at: self.expires_at,
        })
    }
}
