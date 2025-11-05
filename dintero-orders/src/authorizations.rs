use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authorization {
    pub id: String,
    pub order_id: String,
    pub amount: i64,
    pub currency: String,
    pub status: AuthorizationStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_product: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorizationStatus {
    Authorized,
    Expired,
    Cancelled,
    Captured,
    PartiallyCaptured,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateAuthorizationRequest {
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl CreateAuthorizationRequest {
    pub fn new(amount: i64) -> Self {
        Self {
            amount,
            payment_product: None,
            metadata: None,
        }
    }

    pub fn with_payment_product(mut self, product: impl Into<String>) -> Self {
        self.payment_product = Some(product.into());
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationListResponse {
    pub authorizations: Vec<Authorization>,
}
