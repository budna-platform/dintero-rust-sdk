//! Module implementation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct FundTransferRequest {
    pub amount: i64,
    pub currency: String,
    pub from_payout_destination: String,
    pub to_payout_destination: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundTransfer {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub from_payout_destination: String,
    pub to_payout_destination: String,
    pub status: String,
    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl FundTransferRequest {
    pub fn builder() -> FundTransferRequestBuilder {
        FundTransferRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct FundTransferRequestBuilder {
    amount: Option<i64>,
    currency: Option<String>,
    from_payout_destination: Option<String>,
    to_payout_destination: Option<String>,
    reference: Option<String>,
    description: Option<String>,
}

impl FundTransferRequestBuilder {
    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn from_payout_destination(mut self, destination: impl Into<String>) -> Self {
        self.from_payout_destination = Some(destination.into());
        self
    }

    pub fn to_payout_destination(mut self, destination: impl Into<String>) -> Self {
        self.to_payout_destination = Some(destination.into());
        self
    }

    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn build(self) -> Result<FundTransferRequest, String> {
        Ok(FundTransferRequest {
            amount: self.amount.ok_or("amount is required")?,
            currency: self.currency.ok_or("currency is required")?,
            from_payout_destination: self
                .from_payout_destination
                .ok_or("from_payout_destination is required")?,
            to_payout_destination: self
                .to_payout_destination
                .ok_or("to_payout_destination is required")?,
            reference: self.reference,
            description: self.description,
        })
    }
}
