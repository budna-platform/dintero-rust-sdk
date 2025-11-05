use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    pub id: String,
    pub order_id: String,
    pub amount: i64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RefundItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundItem {
    pub line_id: String,
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateRefundRequest {
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RefundItem>>,
}

impl CreateRefundRequest {
    pub fn new(amount: i64) -> Self {
        Self {
            amount,
            reason: None,
            items: None,
        }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn with_items(mut self, items: Vec<RefundItem>) -> Self {
        self.items = Some(items);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundListResponse {
    pub refunds: Vec<Refund>,
}
