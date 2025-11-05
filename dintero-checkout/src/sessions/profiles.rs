//! Module implementation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionProfile {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_shipping_option: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_types: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub express_customer_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_shipping_option: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_types: Option<Vec<String>>,
}

impl CreateProfileRequest {
    pub fn builder() -> CreateProfileRequestBuilder {
        CreateProfileRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateProfileRequestBuilder {
    name: Option<String>,
    logo_url: Option<String>,
    default_shipping_option: Option<String>,
    customer_types: Option<Vec<String>>,
}

impl CreateProfileRequestBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn logo_url(mut self, url: impl Into<String>) -> Self {
        self.logo_url = Some(url.into());
        self
    }

    pub fn default_shipping_option(mut self, option: impl Into<String>) -> Self {
        self.default_shipping_option = Some(option.into());
        self
    }

    pub fn customer_types(mut self, types: Vec<String>) -> Self {
        self.customer_types = Some(types);
        self
    }

    pub fn build(self) -> CreateProfileRequest {
        CreateProfileRequest {
            name: self.name,
            logo_url: self.logo_url,
            default_shipping_option: self.default_shipping_option,
            customer_types: self.customer_types,
        }
    }
}
