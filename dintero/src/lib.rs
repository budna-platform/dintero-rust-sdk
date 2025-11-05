//! # Dintero Rust SDK
//!
//! Official Rust SDK for the Dintero API.
//!
//! This SDK provides a comprehensive interface to all Dintero APIs including Checkout, Orders,
//! Payments, Accounts, Loyalty, and Insights.
//!
//! ## Features
//!
//! - **checkout**: Checkout API for payment sessions
//! - **orders**: Orders API for order management
//! - **payments**: Payments API for payment operations
//! - **accounts**: Accounts API for account management
//! - **loyalty**: Loyalty API for loyalty programs
//! - **insights**: Insights API for analytics and reporting
//!
//! ## Example
//!
//! ```no_run
//! use dintero::{Config, Environment, DinteroClient};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::builder("T12345678")
//!     .api_key("your-api-key")
//!     .environment(Environment::Test)
//!     .build()?;
//!
//! let client = DinteroClient::new(config)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Copyright
//!
//! Copyright (c) 2024 Budna Marketplace AB
//! Author: Marcus Cvjeticanin
//!
//! Licensed under the MIT License.

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

#[cfg(feature = "payments")]
pub mod payments {
    pub use dintero_payments::*;
}

#[cfg(feature = "accounts")]
pub mod accounts {
    pub use dintero_accounts::*;
}

#[cfg(feature = "loyalty")]
pub mod loyalty {
    pub use dintero_loyalty::automations::*;
    pub use dintero_loyalty::customers::*;
    pub use dintero_loyalty::discounts::*;
    pub use dintero_loyalty::locations::*;
    pub use dintero_loyalty::products::*;
    pub use dintero_loyalty::receipts::*;
    pub use dintero_loyalty::types::*;
    pub use dintero_loyalty::wallets::*;
    pub use dintero_loyalty::webhooks::*;
    pub use dintero_loyalty::*;
}

#[cfg(feature = "insights")]
pub mod insights {
    pub use dintero_insights::*;
}

pub use client::HttpClient;
pub use config::{AuthConfig, Config, ConfigBuilder, Environment, RetryConfig};
pub use error::{Error, Result};

use crate::auth::create_auth_provider;
use std::sync::Arc;

/// Main client for interacting with the Dintero API.
///
/// This client provides access to all Dintero API endpoints through feature-gated modules.
/// It handles authentication, retries, and HTTP communication automatically.
///
/// # Examples
///
/// ```no_run
/// use dintero::{Config, Environment, DinteroClient};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = Config::builder("T12345678")
///     .api_key("your-api-key")
///     .environment(Environment::Test)
///     .build()?;
///
/// let client = DinteroClient::new(config)?;
/// # Ok(())
/// # }
/// ```
pub struct DinteroClient {
    http: Arc<HttpClient>,
}

impl DinteroClient {
    /// Creates a new Dintero client with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the client
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub fn new(config: Config) -> Result<Self> {
        let auth = create_auth_provider(&config.auth);
        let http = HttpClient::new(&config, auth)?;

        Ok(Self { http: Arc::new(http) })
    }

    /// Creates a new Dintero client from environment variables.
    ///
    /// Expects the following environment variables:
    /// - `DINTERO_ACCOUNT_ID`: Your Dintero account ID
    /// - `DINTERO_API_KEY`: Your Dintero API key
    /// - `DINTERO_ENVIRONMENT`: Either "production" or "test" (defaults to "test")
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are not set.
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

        let config =
            Config::builder(account_id).environment(environment).api_key(api_key).build()?;

        Self::new(config)
    }

    /// Returns a reference to the underlying HTTP client.
    pub fn http(&self) -> &Arc<HttpClient> {
        &self.http
    }

    /// Returns a checkout client for managing payment sessions.
    ///
    /// Available when the `checkout` feature is enabled.
    #[cfg(feature = "checkout")]
    pub fn checkout(&self) -> checkout::CheckoutClient<adapters::CheckoutHttpAdapter> {
        let adapter = adapters::CheckoutHttpAdapter::new(Arc::clone(&self.http));
        let account_id = self.http.account_id();
        checkout::CheckoutClient::new(adapter, account_id)
    }

    /// Returns an orders client for managing orders.
    ///
    /// Available when the `orders` feature is enabled.
    #[cfg(feature = "orders")]
    pub fn orders(&self) -> orders::OrdersClient<HttpClient> {
        let account_id = self.http.account_id().to_string();
        orders::OrdersClient::new((*self.http).clone(), account_id)
    }

    /// Returns a payments client for managing payment operations.
    ///
    /// Available when the `payments` feature is enabled.
    #[cfg(feature = "payments")]
    pub fn payments(&self) -> payments::PaymentsClient<HttpClient> {
        let account_id = self.http.account_id().to_string();
        payments::PaymentsClient::new((*self.http).clone(), account_id)
    }

    /// Returns an accounts client for account management.
    ///
    /// Available when the `accounts` feature is enabled.
    #[cfg(feature = "accounts")]
    pub fn accounts(&self) -> adapters::accounts::AccountsAdapter {
        adapters::accounts::AccountsAdapter::new(&self.http)
    }

    /// Returns a loyalty client for managing loyalty programs.
    ///
    /// Available when the `loyalty` feature is enabled.
    #[cfg(feature = "loyalty")]
    pub fn loyalty(&self) -> adapters::loyalty::LoyaltyAdapter {
        adapters::loyalty::LoyaltyAdapter::new(self)
    }

    pub(crate) fn http_client(&self) -> &HttpClient {
        &self.http
    }
}

impl Clone for DinteroClient {
    fn clone(&self) -> Self {
        Self { http: Arc::clone(&self.http) }
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
