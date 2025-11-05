pub mod adapters;
pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod types;

#[cfg(feature = "checkout")]
pub mod checkout {
    pub use dintero_checkout::*;
}

#[cfg(feature = "orders")]
pub mod orders {
    pub use dintero_orders::*;
}

pub use client::HttpClient;
pub use config::{AuthConfig, Config, ConfigBuilder, Environment, RetryConfig};
pub use error::{Error, Result};

use crate::auth::create_auth_provider;
use std::sync::Arc;

pub struct DinteroClient {
    http: Arc<HttpClient>,
}

impl DinteroClient {
    pub fn new(config: Config) -> Result<Self> {
        let auth = create_auth_provider(&config.auth);
        let http = HttpClient::new(&config, auth)?;

        Ok(Self {
            http: Arc::new(http),
        })
    }

    pub fn from_env() -> Result<Self> {
        let account_id = std::env::var("DINTERO_ACCOUNT_ID")
            .map_err(|_| error::Error::Config("DINTERO_ACCOUNT_ID not set".to_string()))?;

        let api_key = std::env::var("DINTERO_API_KEY")
            .map_err(|_| error::Error::Config("DINTERO_API_KEY not set".to_string()))?;

        let environment = std::env::var("DINTERO_ENVIRONMENT")
            .map(|env| match env.to_lowercase().as_str() {
                "production" | "prod" => config::Environment::Production,
                _ => config::Environment::Test,
            })
            .unwrap_or(config::Environment::Test);

        let config = Config::builder(account_id)
            .environment(environment)
            .api_key(api_key)
            .build()?;

        Self::new(config)
    }

    pub fn http(&self) -> &Arc<HttpClient> {
        &self.http
    }

    #[cfg(feature = "checkout")]
    pub fn checkout(&self) -> checkout::CheckoutClient<adapters::CheckoutHttpAdapter> {
        let adapter = adapters::CheckoutHttpAdapter::new(Arc::clone(&self.http));
        let account_id = self.http.account_id();
        checkout::CheckoutClient::new(adapter, account_id)
    }

    #[cfg(feature = "orders")]
    pub fn orders(&self) -> orders::OrdersClient<HttpClient> {
        let account_id = self.http.account_id().to_string();
        orders::OrdersClient::new((*self.http).clone(), account_id)
    }
}

impl Clone for DinteroClient {
    fn clone(&self) -> Self {
        Self {
            http: Arc::clone(&self.http),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = Config::builder("T12345678")
            .api_key("test_key")
            .environment(config::Environment::Test)
            .timeout_secs(60)
            .build()
            .unwrap();

        assert_eq!(config.account_id, "T12345678");
        assert_eq!(config.timeout_secs, 60);
    }

    #[test]
    fn test_config_validation() {
        let result = Config::builder("").api_key("test_key").build();

        assert!(result.is_err());
    }
}
