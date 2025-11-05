use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Environment {
    Production,
    #[default]
    Test,
}

impl Environment {
    pub fn base_url(&self) -> &str {
        match self {
            Environment::Production => "https://api.dintero.com",
            Environment::Test => "https://api.test.dintero.com",
        }
    }
}

#[derive(Debug, Clone)]
pub enum AuthConfig {
    ApiKey(String),
    OAuth {
        client_id: String,
        client_secret: String,
    },
    Jwt(String),
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
    pub max_backoff_ms: u64,
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

#[derive(Debug, Clone)]
pub struct Config {
    pub account_id: String,
    pub environment: Environment,
    pub auth: AuthConfig,
    pub timeout_secs: u64,
    pub retry_config: RetryConfig,
}

impl Config {
    pub fn builder(account_id: impl Into<String>) -> ConfigBuilder {
        ConfigBuilder::new(account_id)
    }

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

pub struct ConfigBuilder {
    account_id: String,
    environment: Environment,
    auth: Option<AuthConfig>,
    timeout_secs: u64,
    retry_config: RetryConfig,
}

impl ConfigBuilder {
    pub fn new(account_id: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
            environment: Environment::default(),
            auth: None,
            timeout_secs: 30,
            retry_config: RetryConfig::default(),
        }
    }

    pub fn environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    pub fn auth(mut self, auth: AuthConfig) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn api_key(self, api_key: impl Into<String>) -> Self {
        self.auth(AuthConfig::ApiKey(api_key.into()))
    }

    pub fn oauth(self, client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        self.auth(AuthConfig::OAuth {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        })
    }

    pub fn jwt(self, token: impl Into<String>) -> Self {
        self.auth(AuthConfig::Jwt(token.into()))
    }

    pub fn timeout_secs(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    pub fn retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = retry_config;
        self
    }

    pub fn build(self) -> Result<Config> {
        let auth = self
            .auth
            .ok_or_else(|| Error::Config("auth configuration required".to_string()))?;

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
