use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cancellation {
    pub id: String,
    pub order_id: String,
    pub amount: i64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCancellationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl CreateCancellationRequest {
    pub fn new() -> Self {
        Self {
            amount: None,
            reason: None,
        }
    }

    pub fn with_amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
}

impl Default for CreateCancellationRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancellationListResponse {
    pub cancellations: Vec<Cancellation>,
}
