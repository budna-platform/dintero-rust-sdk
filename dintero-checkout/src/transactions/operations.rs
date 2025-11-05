//! Module implementation.

use super::types::*;
use serde::{Deserialize, Serialize};

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

    pub fn build(self) -> ListTransactionsParams {
        ListTransactionsParams {
            limit: self.limit,
            page_token: self.page_token,
            status: self.status,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTransactionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference_2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl UpdateTransactionRequest {
    pub fn builder() -> UpdateTransactionRequestBuilder {
        UpdateTransactionRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateTransactionRequestBuilder {
    merchant_reference: Option<String>,
    merchant_reference_2: Option<String>,
    metadata: Option<serde_json::Value>,
}

impl UpdateTransactionRequestBuilder {
    pub fn merchant_reference(mut self, reference: impl Into<String>) -> Self {
        self.merchant_reference = Some(reference.into());
        self
    }

    pub fn merchant_reference_2(mut self, reference: impl Into<String>) -> Self {
        self.merchant_reference_2 = Some(reference.into());
        self
    }

    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> UpdateTransactionRequest {
        UpdateTransactionRequest {
            merchant_reference: self.merchant_reference,
            merchant_reference_2: self.merchant_reference_2,
            metadata: self.metadata,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ExtendAuthorizationRequest {
    pub days: u32,
}

impl ExtendAuthorizationRequest {
    pub fn new(days: u32) -> Self {
        Self { days }
    }
}
