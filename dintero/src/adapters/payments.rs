//! Module implementation.

use crate::client::HttpClient;

#[cfg(feature = "payments")]
use async_trait::async_trait;
#[cfg(feature = "payments")]
use dintero_payments::{PaymentsAdapter, Result as PaymentsResult};

#[cfg(feature = "payments")]
#[async_trait]
impl PaymentsAdapter for HttpClient {
    async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> PaymentsResult<T> {
        self.get_json(path)
            .await
            .map_err(|e| dintero_payments::PaymentsError::Client(e.to_string()))
    }

    async fn post_json<T: serde::de::DeserializeOwned, B: serde::Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> PaymentsResult<T> {
        self.post_json(path, body)
            .await
            .map_err(|e| dintero_payments::PaymentsError::Client(e.to_string()))
    }

    async fn put_json<T: serde::de::DeserializeOwned, B: serde::Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> PaymentsResult<T> {
        self.put_json(path, body)
            .await
            .map_err(|e| dintero_payments::PaymentsError::Client(e.to_string()))
    }

    async fn delete(&self, path: &str) -> PaymentsResult<()> {
        self.delete_request(path)
            .await
            .map_err(|e| dintero_payments::PaymentsError::Client(e.to_string()))
    }
}
