use crate::client::HttpClient as DinteroHttpClient;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

pub struct CheckoutHttpAdapter {
    http: Arc<DinteroHttpClient>,
}

impl CheckoutHttpAdapter {
    pub fn new(http: Arc<DinteroHttpClient>) -> Self {
        Self { http }
    }
}

#[async_trait]
impl dintero_checkout::HttpClient for CheckoutHttpAdapter {
    async fn post_json<B: Serialize + Send + Sync, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> dintero_checkout::Result<T> {
        let builder = self
            .http
            .post(path)
            .await
            .map_err(|e| dintero_checkout::CheckoutError::Client(e.to_string()))?;

        self.http
            .send_json(builder, body)
            .await
            .map_err(|e| dintero_checkout::CheckoutError::Client(e.to_string()))
    }

    async fn get_json<T: DeserializeOwned>(&self, path: &str) -> dintero_checkout::Result<T> {
        let builder = self
            .http
            .get(path)
            .await
            .map_err(|e| dintero_checkout::CheckoutError::Client(e.to_string()))?;

        self.http
            .send(builder)
            .await
            .map_err(|e| dintero_checkout::CheckoutError::Client(e.to_string()))
    }

    async fn put_json<B: Serialize + Send + Sync, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> dintero_checkout::Result<T> {
        let builder = self
            .http
            .put(path)
            .await
            .map_err(|e| dintero_checkout::CheckoutError::Client(e.to_string()))?;

        self.http
            .send_json(builder, body)
            .await
            .map_err(|e| dintero_checkout::CheckoutError::Client(e.to_string()))
    }

    async fn delete(&self, path: &str) -> dintero_checkout::Result<()> {
        let builder = self
            .http
            .delete(path)
            .await
            .map_err(|e| dintero_checkout::CheckoutError::Client(e.to_string()))?;

        self.http
            .send_empty(builder)
            .await
            .map_err(|e| dintero_checkout::CheckoutError::Client(e.to_string()))
    }
}
