//! Module implementation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Authorized,
    Captured,
    PartiallyRefunded,
    Refunded,
    Voided,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub status: TransactionStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference_2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_product_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CaptureTransactionRequest {
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TransactionItem>>,
}

impl CaptureTransactionRequest {
    pub fn new(amount: i64) -> Self {
        Self { amount, items: None }
    }

    pub fn with_items(mut self, items: Vec<TransactionItem>) -> Self {
        self.items = Some(items);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionItem {
    pub line_id: String,
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RefundTransactionRequest {
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TransactionItem>>,
}

impl RefundTransactionRequest {
    pub fn new(amount: i64) -> Self {
        Self { amount, reason: None, items: None }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn with_items(mut self, items: Vec<TransactionItem>) -> Self {
        self.items = Some(items);
        self
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct VoidTransactionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl VoidTransactionRequest {
    pub fn new() -> Self {
        Self { reason: None }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
}

impl Default for VoidTransactionRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateTransactionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference_2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl UpdateTransactionRequest {
    pub fn new() -> Self {
        Self {
            merchant_reference: None,
            merchant_reference_2: None,
            metadata: None,
        }
    }

    pub fn with_merchant_reference(mut self, reference: impl Into<String>) -> Self {
        self.merchant_reference = Some(reference.into());
        self
    }

    pub fn with_merchant_reference_2(mut self, reference: impl Into<String>) -> Self {
        self.merchant_reference_2 = Some(reference.into());
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl Default for UpdateTransactionRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ExtendAuthorizationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_days: Option<i32>,
}

impl ExtendAuthorizationRequest {
    pub fn new(extend_days: i32) -> Self {
        Self { extend_days: Some(extend_days) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionListResponse {
    pub transactions: Vec<Transaction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ListTransactionsParams {
    pub limit: Option<u32>,
    pub page_token: Option<String>,
    pub status: Option<TransactionStatus>,
    pub merchant_reference: Option<String>,
}

impl ListTransactionsParams {
    pub fn builder() -> ListTransactionsParamsBuilder {
        ListTransactionsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ListTransactionsParamsBuilder {
    limit: Option<u32>,
    page_token: Option<String>,
    status: Option<TransactionStatus>,
    merchant_reference: Option<String>,
}

impl ListTransactionsParamsBuilder {
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.page_token = Some(token.into());
        self
    }

    pub fn status(mut self, status: TransactionStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn merchant_reference(mut self, reference: impl Into<String>) -> Self {
        self.merchant_reference = Some(reference.into());
        self
    }

    pub fn build(self) -> ListTransactionsParams {
        ListTransactionsParams {
            limit: self.limit,
            page_token: self.page_token,
            status: self.status,
            merchant_reference: self.merchant_reference,
        }
    }
}
