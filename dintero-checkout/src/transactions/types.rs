use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    OnHold,
    Authorized,
    PartiallySettled,
    Settled,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub session_id: String,
    pub status: TransactionStatus,
    pub amount: i64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_product: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureRequest {
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CaptureItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureItem {
    pub line_id: String,
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    pub amount: i64,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoidRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl CaptureRequest {
    pub fn new(amount: i64) -> Self {
        Self {
            amount,
            items: None,
        }
    }

    pub fn with_items(mut self, items: Vec<CaptureItem>) -> Self {
        self.items = Some(items);
        self
    }
}

impl RefundRequest {
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

impl VoidRequest {
    pub fn new() -> Self {
        Self { reason: None }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
}

impl Default for VoidRequest {
    fn default() -> Self {
        Self::new()
    }
}
