use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Created,
    Started,
    Authorized,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutSession {
    pub id: String,
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SessionStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub amount: i64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OrderItem>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
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
    pub discount_amount: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_place: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

pub type BillingAddress = ShippingAddress;

#[derive(Debug, Clone)]
pub struct CreateSessionRequest {
    pub url: SessionUrl,
    pub order: Order,
    pub profile_id: Option<String>,
    pub return_url: Option<String>,
    pub merchant_terms_url: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SessionUrl {
    pub return_url: Option<String>,
    pub callback_url: Option<String>,
}

impl CreateSessionRequest {
    pub fn builder() -> CreateSessionRequestBuilder {
        CreateSessionRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateSessionRequestBuilder {
    url: SessionUrl,
    order: Option<Order>,
    profile_id: Option<String>,
    return_url: Option<String>,
    merchant_terms_url: Option<String>,
}

impl CreateSessionRequestBuilder {
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    pub fn return_url(mut self, url: impl Into<String>) -> Self {
        self.url.return_url = Some(url.into());
        self
    }

    pub fn callback_url(mut self, url: impl Into<String>) -> Self {
        self.url.callback_url = Some(url.into());
        self
    }

    pub fn profile_id(mut self, id: impl Into<String>) -> Self {
        self.profile_id = Some(id.into());
        self
    }

    pub fn merchant_terms_url(mut self, url: impl Into<String>) -> Self {
        self.merchant_terms_url = Some(url.into());
        self
    }

    pub fn build(self) -> Result<CreateSessionRequest, String> {
        let order = self.order.ok_or("order is required")?;

        Ok(CreateSessionRequest {
            url: self.url,
            order,
            profile_id: self.profile_id,
            return_url: self.return_url,
            merchant_terms_url: self.merchant_terms_url,
        })
    }
}

impl Order {
    pub fn builder() -> OrderBuilder {
        OrderBuilder::default()
    }
}

#[derive(Default)]
pub struct OrderBuilder {
    amount: i64,
    currency: String,
    merchant_reference: Option<String>,
    items: Vec<OrderItem>,
    vat_amount: Option<i64>,
    shipping_address: Option<ShippingAddress>,
    billing_address: Option<BillingAddress>,
}

impl OrderBuilder {
    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = amount;
        self
    }

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = currency.into();
        self
    }

    pub fn merchant_reference(mut self, reference: impl Into<String>) -> Self {
        self.merchant_reference = Some(reference.into());
        self
    }

    pub fn add_item(mut self, item: OrderItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn vat_amount(mut self, amount: i64) -> Self {
        self.vat_amount = Some(amount);
        self
    }

    pub fn shipping_address(mut self, address: ShippingAddress) -> Self {
        self.shipping_address = Some(address);
        self
    }

    pub fn billing_address(mut self, address: BillingAddress) -> Self {
        self.billing_address = Some(address);
        self
    }

    pub fn build(self) -> Order {
        Order {
            amount: self.amount,
            currency: self.currency,
            merchant_reference: self.merchant_reference,
            items: if self.items.is_empty() {
                None
            } else {
                Some(self.items)
            },
            vat_amount: self.vat_amount,
            shipping_address: self.shipping_address,
            billing_address: self.billing_address,
        }
    }
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
            discount_amount: None,
        }
    }

    pub fn with_discount(mut self, discount_amount: i64) -> Self {
        self.discount_amount = Some(discount_amount);
        self
    }
}
