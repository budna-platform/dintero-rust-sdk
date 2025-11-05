use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Created,
    Authorized,
    PartiallyAuthorized,
    Captured,
    PartiallyCaptured,
    PartiallyRefunded,
    Refunded,
    Cancelled,
    PartiallyCancelled,
    Closed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub status: OrderStatus,
    pub amount: i64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference_2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OrderItem>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: String,
    pub line_id: String,
    pub description: String,
    pub quantity: i64,
    pub amount: i64,
    pub vat_amount: i64,
    pub vat: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_percentage: Option<i64>,
}

impl OrderItem {
    pub fn new(
        id: impl Into<String>,
        line_id: impl Into<String>,
        description: impl Into<String>,
        quantity: i64,
        amount: i64,
        vat_amount: i64,
        vat: i64,
    ) -> Self {
        Self {
            id: id.into(),
            line_id: line_id.into(),
            description: description.into(),
            quantity,
            amount,
            vat_amount,
            vat,
            product_id: None,
            discount_amount: None,
            discount_percentage: None,
        }
    }

    pub fn with_product_id(mut self, product_id: impl Into<String>) -> Self {
        self.product_id = Some(product_id.into());
        self
    }

    pub fn with_discount_amount(mut self, amount: i64) -> Self {
        self.discount_amount = Some(amount);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_place: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateOrderRequest {
    pub amount: i64,
    pub currency: String,
    pub items: Vec<OrderItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference_2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl CreateOrderRequest {
    pub fn builder() -> CreateOrderRequestBuilder {
        CreateOrderRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateOrderRequestBuilder {
    amount: Option<i64>,
    currency: Option<String>,
    items: Vec<OrderItem>,
    merchant_reference: Option<String>,
    merchant_reference_2: Option<String>,
    customer: Option<Customer>,
    shipping_address: Option<Address>,
    billing_address: Option<Address>,
    metadata: Option<serde_json::Value>,
}

impl CreateOrderRequestBuilder {
    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn add_item(mut self, item: OrderItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: Vec<OrderItem>) -> Self {
        self.items = items;
        self
    }

    pub fn merchant_reference(mut self, reference: impl Into<String>) -> Self {
        self.merchant_reference = Some(reference.into());
        self
    }

    pub fn merchant_reference_2(mut self, reference: impl Into<String>) -> Self {
        self.merchant_reference_2 = Some(reference.into());
        self
    }

    pub fn customer(mut self, customer: Customer) -> Self {
        self.customer = Some(customer);
        self
    }

    pub fn shipping_address(mut self, address: Address) -> Self {
        self.shipping_address = Some(address);
        self
    }

    pub fn billing_address(mut self, address: Address) -> Self {
        self.billing_address = Some(address);
        self
    }

    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<CreateOrderRequest, String> {
        Ok(CreateOrderRequest {
            amount: self.amount.ok_or("amount is required")?,
            currency: self.currency.ok_or("currency is required")?,
            items: self.items,
            merchant_reference: self.merchant_reference,
            merchant_reference_2: self.merchant_reference_2,
            customer: self.customer,
            shipping_address: self.shipping_address,
            billing_address: self.billing_address,
            metadata: self.metadata,
        })
    }
}
