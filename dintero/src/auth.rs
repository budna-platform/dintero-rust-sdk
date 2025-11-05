//! Authentication providers for the Dintero API.
//!
//! This module provides different authentication mechanisms including API keys,
//! OAuth2, and JWT tokens.

use crate::config::AuthConfig;
use crate::error::{Error, Result};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Trait for authentication providers.
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Returns the authentication header value for HTTP requests.
    async fn get_auth_header(&self) -> Result<String>;
}

/// API key authentication provider.
pub struct ApiKeyAuth {
    api_key: String,
}

impl ApiKeyAuth {
    /// Creates a new API key authentication provider.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self { api_key: api_key.into() }
    }
}

#[async_trait]
impl AuthProvider for ApiKeyAuth {
    async fn get_auth_header(&self) -> Result<String> {
        Ok(format!("Token {}", self.api_key))
    }
}

/// OAuth2 authentication provider.
pub struct OAuthAuth {
    #[allow(dead_code)]
    client_id: String,
    #[allow(dead_code)]
    client_secret: String,
    token: Arc<RwLock<Option<OAuthToken>>>,
}

#[derive(Clone)]
struct OAuthToken {
    access_token: String,
    expires_at: std::time::Instant,
}

impl OAuthAuth {
    /// Creates a new OAuth2 authentication provider.
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            token: Arc::new(RwLock::new(None)),
        }
    }

    async fn refresh_token_if_needed(&self) -> Result<()> {
        let token = self.token.read().await;

        if let Some(token) = token.as_ref() {
            if token.expires_at > std::time::Instant::now() {
                return Ok(());
            }
        }

        drop(token);

        let mut token = self.token.write().await;

        if let Some(token) = token.as_ref() {
            if token.expires_at > std::time::Instant::now() {
                return Ok(());
            }
        }

        let new_token = self.fetch_token().await?;
        *token = Some(new_token);
        Ok(())
    }

    async fn fetch_token(&self) -> Result<OAuthToken> {
        Err(Error::Auth(
            "OAuth token exchange not yet implemented".to_string(),
        ))
    }
}

#[async_trait]
impl AuthProvider for OAuthAuth {
    async fn get_auth_header(&self) -> Result<String> {
        self.refresh_token_if_needed().await?;

        let token = self.token.read().await;
        let access_token = token
            .as_ref()
            .ok_or_else(|| Error::Auth("No access token available".to_string()))?
            .access_token
            .clone();

        Ok(format!("Bearer {}", access_token))
    }
}

/// JWT token authentication provider.
pub struct JwtAuth {
    token: String,
}

impl JwtAuth {
    /// Creates a new JWT authentication provider.
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }
}

#[async_trait]
impl AuthProvider for JwtAuth {
    async fn get_auth_header(&self) -> Result<String> {
        Ok(format!("Bearer {}", self.token))
    }
}

/// Creates an authentication provider from configuration.
pub fn create_auth_provider(config: &AuthConfig) -> Arc<dyn AuthProvider> {
    match config {
        AuthConfig::ApiKey(key) => Arc::new(ApiKeyAuth::new(key.clone())),
        AuthConfig::OAuth { client_id, client_secret } => {
            Arc::new(OAuthAuth::new(client_id.clone(), client_secret.clone()))
        }
        AuthConfig::Jwt(token) => Arc::new(JwtAuth::new(token.clone())),
    }
}
