//! Module implementation.

use crate::orders::{Address, Customer};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftOrder {
    pub id: String,
    pub amount: i64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DraftOrderItem>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftOrderItem {
    pub line_id: String,
    pub description: String,
    pub quantity: i64,
    pub amount: i64,
    pub vat_amount: i64,
    pub vat: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateDraftOrderRequest {
    pub amount: i64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DraftOrderItem>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
}

impl CreateDraftOrderRequest {
    pub fn builder() -> CreateDraftOrderRequestBuilder {
        CreateDraftOrderRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateDraftOrderRequestBuilder {
    amount: Option<i64>,
    currency: Option<String>,
    merchant_reference: Option<String>,
    items: Vec<DraftOrderItem>,
    customer: Option<Customer>,
}

impl CreateDraftOrderRequestBuilder {
    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn merchant_reference(mut self, reference: impl Into<String>) -> Self {
        self.merchant_reference = Some(reference.into());
        self
    }

    pub fn add_item(mut self, item: DraftOrderItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn customer(mut self, customer: Customer) -> Self {
        self.customer = Some(customer);
        self
    }

    pub fn build(self) -> Result<CreateDraftOrderRequest, String> {
        Ok(CreateDraftOrderRequest {
            amount: self.amount.ok_or("amount is required")?,
            currency: self.currency.ok_or("currency is required")?,
            merchant_reference: self.merchant_reference,
            items: if self.items.is_empty() { None } else { Some(self.items) },
            customer: self.customer,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDraftOrderItemRequest {
    pub quantity: i64,
    pub amount: i64,
}

impl UpdateDraftOrderItemRequest {
    pub fn new(quantity: i64, amount: i64) -> Self {
        Self { quantity, amount }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AddDraftOrderItemRequest {
    pub line_id: String,
    pub description: String,
    pub quantity: i64,
    pub amount: i64,
    pub vat_amount: i64,
    pub vat: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

impl AddDraftOrderItemRequest {
    pub fn new(
        line_id: impl Into<String>,
        description: impl Into<String>,
        quantity: i64,
        amount: i64,
        vat_amount: i64,
        vat: i64,
    ) -> Self {
        Self {
            line_id: line_id.into(),
            description: description.into(),
            quantity,
            amount,
            vat_amount,
            vat,
            product_id: None,
        }
    }

    pub fn with_product_id(mut self, product_id: impl Into<String>) -> Self {
        self.product_id = Some(product_id.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftOrderListResponse {
    pub draft_orders: Vec<DraftOrder>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}
