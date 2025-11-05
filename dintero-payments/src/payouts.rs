//! Module implementation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutDestination {
    pub id: String,
    pub account_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutDestinationListResponse {
    pub payout_destinations: Vec<PayoutDestination>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreatePayoutDestinationRequest {
    pub name: String,
    pub account_number: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
}

impl CreatePayoutDestinationRequest {
    pub fn new(name: impl Into<String>, account_number: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            account_number: account_number.into(),
            bank_code: None,
        }
    }

    pub fn with_bank_code(mut self, code: impl Into<String>) -> Self {
        self.bank_code = Some(code.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutBalance {
    pub currency: String,
    pub available: i64,
    pub pending: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutTransfer {
    pub id: String,
    pub amount: i64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_destination_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutTransferListResponse {
    pub transfers: Vec<PayoutTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreatePayoutTransferRequest {
    pub amount: i64,
    pub currency: String,
    pub payout_destination_id: String,
}

impl CreatePayoutTransferRequest {
    pub fn new(
        amount: i64,
        currency: impl Into<String>,
        payout_destination_id: impl Into<String>,
    ) -> Self {
        Self {
            amount,
            currency: currency.into(),
            payout_destination_id: payout_destination_id.into(),
        }
    }
}
