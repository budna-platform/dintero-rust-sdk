//! Module implementation.

use super::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderListResponse {
    pub orders: Vec<Order>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ListOrdersParams {
    pub limit: Option<u32>,
    pub page_token: Option<String>,
    pub status: Option<OrderStatus>,
    pub merchant_reference: Option<String>,
}

impl ListOrdersParams {
    pub fn builder() -> ListOrdersParamsBuilder {
        ListOrdersParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ListOrdersParamsBuilder {
    limit: Option<u32>,
    page_token: Option<String>,
    status: Option<OrderStatus>,
    merchant_reference: Option<String>,
}

impl ListOrdersParamsBuilder {
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.page_token = Some(token.into());
        self
    }

    pub fn status(mut self, status: OrderStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn merchant_reference(mut self, reference: impl Into<String>) -> Self {
        self.merchant_reference = Some(reference.into());
        self
    }

    pub fn build(self) -> ListOrdersParams {
        ListOrdersParams {
            limit: self.limit,
            page_token: self.page_token,
            status: self.status,
            merchant_reference: self.merchant_reference,
        }
    }
}
