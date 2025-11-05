//! Module implementation.

use crate::client::HttpClient;

#[cfg(feature = "orders")]
use async_trait::async_trait;
#[cfg(feature = "orders")]
use dintero_orders::{OrdersAdapter, Result as OrdersResult};

#[cfg(feature = "orders")]
#[async_trait]
impl OrdersAdapter for HttpClient {
    async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> OrdersResult<T> {
        self.get_json(path).await.map_err(|e| dintero_orders::OrdersError::Client(e.to_string()))
    }

    async fn post_json<T: serde::de::DeserializeOwned, B: serde::Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> OrdersResult<T> {
        self.post_json(path, body)
            .await
            .map_err(|e| dintero_orders::OrdersError::Client(e.to_string()))
    }

    async fn put_json<T: serde::de::DeserializeOwned, B: serde::Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> OrdersResult<T> {
        self.put_json(path, body)
            .await
            .map_err(|e| dintero_orders::OrdersError::Client(e.to_string()))
    }

    async fn delete(&self, path: &str) -> OrdersResult<()> {
        self.delete_request(path)
            .await
            .map_err(|e| dintero_orders::OrdersError::Client(e.to_string()))
    }
}
