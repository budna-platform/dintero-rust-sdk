//! Credit check operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct CreditCheckRequest {
    pub customer: CreditCheckCustomer,
    pub order: CreditCheckOrder,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCheckCustomer {
    pub email: String,
    pub phone_number: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreditCheckAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCheckAddress {
    pub country: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_place: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCheckOrder {
    pub amount: i64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreditCheckItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCheckItem {
    pub id: String,
    pub description: String,
    pub quantity: i32,
    pub amount: i64,
    pub vat_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCheckResponse {
    pub approved: bool,
    pub credit_limit: Option<i64>,
    pub provider: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl CreditCheckRequest {
    pub fn builder() -> CreditCheckRequestBuilder {
        CreditCheckRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CreditCheckRequestBuilder {
    customer: Option<CreditCheckCustomer>,
    order: Option<CreditCheckOrder>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl CreditCheckRequestBuilder {
    pub fn customer(mut self, customer: CreditCheckCustomer) -> Self {
        self.customer = Some(customer);
        self
    }

    pub fn order(mut self, order: CreditCheckOrder) -> Self {
        self.order = Some(order);
        self
    }

    pub fn metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<CreditCheckRequest, String> {
        Ok(CreditCheckRequest {
            customer: self.customer.ok_or("customer is required")?,
            order: self.order.ok_or("order is required")?,
            metadata: self.metadata,
        })
    }
}
