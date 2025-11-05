use crate::auth::AuthProvider;
use crate::config::Config;
use crate::error::{Error, Result};
use reqwest::{header, Client, Method, RequestBuilder, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn};

pub struct HttpClient {
    client: Client,
    auth: Arc<dyn AuthProvider>,
    base_url: String,
    account_id: String,
    max_retries: u32,
    initial_backoff_ms: u64,
    max_backoff_ms: u64,
    backoff_multiplier: f64,
}

impl HttpClient {
    pub fn new(config: &Config, auth: Arc<dyn AuthProvider>) -> Result<Self> {
        let timeout = Duration::from_secs(config.timeout_secs);

        let client = Client::builder()
            .timeout(timeout)
            .connection_verbose(true)
            .pool_max_idle_per_host(10)
            .build()
            .map_err(|e| Error::Config(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            auth,
            base_url: config.environment.base_url().to_string(),
            account_id: config.account_id.clone(),
            max_retries: config.retry_config.max_retries,
            initial_backoff_ms: config.retry_config.initial_backoff_ms,
            max_backoff_ms: config.retry_config.max_backoff_ms,
            backoff_multiplier: config.retry_config.backoff_multiplier,
        })
    }

    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    fn build_url(&self, path: &str) -> String {
        let path = path.trim_start_matches('/');
        format!("{}/v1/{}", self.base_url, path)
    }

    async fn add_auth_header(&self, builder: RequestBuilder) -> Result<RequestBuilder> {
        let auth_header = self.auth.get_auth_header().await?;
        Ok(builder.header(header::AUTHORIZATION, auth_header))
    }

    pub async fn request(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let url = self.build_url(path);
        let builder = self.client.request(method, url);
        let builder = builder.header(header::CONTENT_TYPE, "application/json");
        self.add_auth_header(builder).await
    }

    pub async fn get(&self, path: &str) -> Result<RequestBuilder> {
        self.request(Method::GET, path).await
    }

    pub async fn post(&self, path: &str) -> Result<RequestBuilder> {
        self.request(Method::POST, path).await
    }

    pub async fn put(&self, path: &str) -> Result<RequestBuilder> {
        self.request(Method::PUT, path).await
    }

    pub async fn delete(&self, path: &str) -> Result<RequestBuilder> {
        self.request(Method::DELETE, path).await
    }

    pub async fn patch(&self, path: &str) -> Result<RequestBuilder> {
        self.request(Method::PATCH, path).await
    }

    async fn execute_with_retry(&self, builder: RequestBuilder) -> Result<Response> {
        let mut attempts = 0;
        let mut backoff = self.initial_backoff_ms;

        loop {
            let request = builder.try_clone().ok_or_else(|| {
                Error::Validation("Request could not be cloned for retry".to_string())
            })?;

            match request.send().await {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        debug!("Request succeeded with status: {}", status);
                        return Ok(response);
                    }

                    if status == StatusCode::TOO_MANY_REQUESTS {
                        let retry_after = response
                            .headers()
                            .get(header::RETRY_AFTER)
                            .and_then(|v| v.to_str().ok())
                            .and_then(|v| v.parse::<u64>().ok())
                            .map(Duration::from_secs);

                        if attempts >= self.max_retries {
                            return Err(Error::RateLimited { retry_after });
                        }

                        let wait_time = retry_after.unwrap_or(Duration::from_millis(backoff));
                        warn!("Rate limited, retrying after {:?}", wait_time);
                        tokio::time::sleep(wait_time).await;
                        attempts += 1;
                        backoff = (backoff as f64 * self.backoff_multiplier)
                            .min(self.max_backoff_ms as f64)
                            as u64;
                        continue;
                    }

                    if status.is_server_error() && attempts < self.max_retries {
                        warn!("Server error ({}), retrying in {}ms", status, backoff);
                        tokio::time::sleep(Duration::from_millis(backoff)).await;
                        attempts += 1;
                        backoff = (backoff as f64 * self.backoff_multiplier)
                            .min(self.max_backoff_ms as f64)
                            as u64;
                        continue;
                    }

                    let error_body = response.text().await.unwrap_or_default();

                    if let Ok(api_error) = serde_json::from_str::<ApiError>(&error_body) {
                        return Err(Error::Api {
                            code: api_error.error.code,
                            message: api_error.error.message,
                        });
                    }

                    return Err(Error::Api {
                        code: status.as_str().to_string(),
                        message: error_body,
                    });
                }
                Err(e) if e.is_timeout() && attempts < self.max_retries => {
                    warn!("Request timeout, retrying in {}ms", backoff);
                    tokio::time::sleep(Duration::from_millis(backoff)).await;
                    attempts += 1;
                    backoff = (backoff as f64 * self.backoff_multiplier)
                        .min(self.max_backoff_ms as f64) as u64;
                    continue;
                }
                Err(e) => return Err(Error::Http(e)),
            }
        }
    }

    pub async fn send<T: DeserializeOwned>(&self, builder: RequestBuilder) -> Result<T> {
        let response = self.execute_with_retry(builder).await?;
        let body = response.bytes().await?;

        if body.is_empty() {
            return Err(Error::Api {
                code: "EMPTY_RESPONSE".to_string(),
                message: "Expected JSON response but got empty body".to_string(),
            });
        }

        serde_json::from_slice(&body).map_err(Error::from)
    }

    pub async fn send_json<B: Serialize, T: DeserializeOwned>(
        &self,
        builder: RequestBuilder,
        body: &B,
    ) -> Result<T> {
        let builder = builder.json(body);
        self.send(builder).await
    }

    pub async fn send_empty(&self, builder: RequestBuilder) -> Result<()> {
        self.execute_with_retry(builder).await?;
        Ok(())
    }
}

#[derive(serde::Deserialize)]
struct ApiError {
    error: ApiErrorDetail,
}

#[derive(serde::Deserialize)]
struct ApiErrorDetail {
    code: String,
    message: String,
}
