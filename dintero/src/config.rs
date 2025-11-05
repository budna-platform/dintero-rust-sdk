//! Configuration types for the Dintero SDK.

use crate::error::{Error, Result};

/// Dintero API environment.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Environment {
    /// Production environment.
    Production,
    /// Test/Sandbox environment.
    #[default]
    Test,
}

impl Environment {
    /// Returns the base URL for this environment.
    pub fn base_url(&self) -> &str {
        match self {
            Environment::Production => "https://api.dintero.com",
            Environment::Test => "https://api.test.dintero.com",
        }
    }
}

/// Authentication configuration.
#[derive(Debug, Clone)]
pub enum AuthConfig {
    /// API key authentication.
    ApiKey(String),
    /// OAuth2 client credentials.
    OAuth {
        /// OAuth client ID.
        client_id: String,
        /// OAuth client secret.
        client_secret: String,
    },
    /// JWT token authentication.
    Jwt(String),
}

/// Retry configuration for failed requests.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts.
    pub max_retries: u32,
    /// Initial backoff duration in milliseconds.
    pub initial_backoff_ms: u64,
    /// Maximum backoff duration in milliseconds.
    pub max_backoff_ms: u64,
    /// Backoff multiplier for exponential backoff.
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff_ms: 100,
            max_backoff_ms: 10_000,
            backoff_multiplier: 2.0,
        }
    }
}

/// Main configuration for the Dintero SDK.
#[derive(Debug, Clone)]
pub struct Config {
    /// Dintero account ID (e.g., "T12345678").
    pub account_id: String,
    /// API environment to use.
    pub environment: Environment,
    /// Authentication configuration.
    pub auth: AuthConfig,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
    /// Retry configuration.
    pub retry_config: RetryConfig,
}

impl Config {
    /// Creates a new configuration builder.
    pub fn builder(account_id: impl Into<String>) -> ConfigBuilder {
        ConfigBuilder::new(account_id)
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<()> {
        if self.account_id.is_empty() {
            return Err(Error::Config("account_id cannot be empty".to_string()));
        }
        if self.timeout_secs == 0 {
            return Err(Error::Config(
                "timeout_secs must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }
}

/// Builder for creating a Dintero SDK configuration.
pub struct ConfigBuilder {
    account_id: String,
    environment: Environment,
    auth: Option<AuthConfig>,
    timeout_secs: u64,
    retry_config: RetryConfig,
}

impl ConfigBuilder {
    /// Creates a new configuration builder.
    pub fn new(account_id: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
            environment: Environment::default(),
            auth: None,
            timeout_secs: 30,
            retry_config: RetryConfig::default(),
        }
    }

    /// Sets the API environment.
    pub fn environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    /// Sets the authentication configuration.
    pub fn auth(mut self, auth: AuthConfig) -> Self {
        self.auth = Some(auth);
        self
    }

    /// Sets API key authentication.
    pub fn api_key(self, api_key: impl Into<String>) -> Self {
        self.auth(AuthConfig::ApiKey(api_key.into()))
    }

    /// Sets OAuth2 authentication.
    pub fn oauth(self, client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        self.auth(AuthConfig::OAuth {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        })
    }

    /// Sets JWT token authentication.
    pub fn jwt(self, token: impl Into<String>) -> Self {
        self.auth(AuthConfig::Jwt(token.into()))
    }

    /// Sets the request timeout in seconds.
    pub fn timeout_secs(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    /// Sets the retry configuration.
    pub fn retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = retry_config;
        self
    }

    /// Builds the configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing or validation fails.
    pub fn build(self) -> Result<Config> {
        let auth =
            self.auth.ok_or_else(|| Error::Config("auth configuration required".to_string()))?;

        let config = Config {
            account_id: self.account_id,
            environment: self.environment,
            auth,
            timeout_secs: self.timeout_secs,
            retry_config: self.retry_config,
        };

        config.validate()?;
        Ok(config)
    }
}
