//! Module implementation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellerBalance {
    pub payout_destination: String,
    pub available_balance: i64,
    pub pending_balance: i64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellerTransfer {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub payout_destination: String,
    pub status: String,
    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub settled_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ListSellerTransfersParams {
    pub limit: Option<u32>,
    pub page_token: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

impl ListSellerTransfersParams {
    pub fn builder() -> ListSellerTransfersParamsBuilder {
        ListSellerTransfersParamsBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ListSellerTransfersParamsBuilder {
    limit: Option<u32>,
    page_token: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
}

impl ListSellerTransfersParamsBuilder {
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.page_token = Some(token.into());
        self
    }

    pub fn from_date(mut self, date: impl Into<String>) -> Self {
        self.from_date = Some(date.into());
        self
    }

    pub fn to_date(mut self, date: impl Into<String>) -> Self {
        self.to_date = Some(date.into());
        self
    }

    pub fn build(self) -> ListSellerTransfersParams {
        ListSellerTransfersParams {
            limit: self.limit,
            page_token: self.page_token,
            from_date: self.from_date,
            to_date: self.to_date,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellerTransfersResponse {
    pub transfers: Vec<SellerTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}
