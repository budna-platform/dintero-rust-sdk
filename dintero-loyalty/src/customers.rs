//! Customer management in loyalty programs.

use crate::client::LoyaltyClient;
use crate::error::Result;
use crate::types::{Address, CustomerStatus, CustomerType, PaginatedResponse, PhoneNumber};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(rename = "type")]
    pub customer_type: CustomerType,
    pub status: CustomerStatus,
    pub phone_number: Option<PhoneNumber>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub organization_number: Option<String>,
    pub addresses: Option<Vec<Address>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    #[serde(rename = "type")]
    pub customer_type: CustomerType,
    pub phone_number: Option<PhoneNumber>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub organization_number: Option<String>,
    pub addresses: Option<Vec<Address>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerRequest {
    pub phone_number: Option<PhoneNumber>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub organization_number: Option<String>,
    pub addresses: Option<Vec<Address>>,
    pub status: Option<CustomerStatus>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
pub struct ListCustomersRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub query: Option<String>,
    pub status: Option<CustomerStatus>,
    pub customer_type: Option<CustomerType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerToken {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerTokenRequest {
    pub expires_in_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerTerms {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub terms_id: String,
    pub accepted_at: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptTermsRequest {
    pub terms_id: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSettings {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub notifications_enabled: bool,
    pub marketing_enabled: bool,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerSettingsRequest {
    pub notifications_enabled: Option<bool>,
    pub marketing_enabled: Option<bool>,
    pub language: Option<String>,
}

impl LoyaltyClient {
    pub async fn create_customer(&self, req: CreateCustomerRequest) -> Result<Customer> {
        let url = self.url("/customers");
        let response = self.http().post(&url).json(&req).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn get_customer(&self, customer_id: &Uuid) -> Result<Customer> {
        let url = self.url(&format!("/customers/{}", customer_id));
        let response = self.http().get(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn update_customer(
        &self,
        customer_id: &Uuid,
        req: UpdateCustomerRequest,
    ) -> Result<Customer> {
        let url = self.url(&format!("/customers/{}", customer_id));
        let response = self.http().put(&url).json(&req).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn delete_customer(&self, customer_id: &Uuid) -> Result<()> {
        let url = self.url(&format!("/customers/{}", customer_id));
        let response = self.http().delete(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn list_customers(
        &self,
        req: ListCustomersRequest,
    ) -> Result<PaginatedResponse<Customer>> {
        let mut url = self.url("/customers");
        let mut params = vec![];

        if let Some(limit) = req.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = req.offset {
            params.push(format!("offset={}", offset));
        }
        if let Some(query) = &req.query {
            params.push(format!("query={}", urlencoding::encode(query)));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.http().get(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn create_customer_token(
        &self,
        customer_id: &Uuid,
        req: CreateCustomerTokenRequest,
    ) -> Result<CustomerToken> {
        let url = self.url(&format!("/customers/{}/tokens", customer_id));
        let response = self.http().post(&url).json(&req).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn accept_terms(
        &self,
        customer_id: &Uuid,
        req: AcceptTermsRequest,
    ) -> Result<CustomerTerms> {
        let url = self.url(&format!("/customers/{}/terms", customer_id));
        let response = self.http().post(&url).json(&req).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn get_customer_settings(&self, customer_id: &Uuid) -> Result<CustomerSettings> {
        let url = self.url(&format!("/customers/{}/settings", customer_id));
        let response = self.http().get(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn update_customer_settings(
        &self,
        customer_id: &Uuid,
        req: UpdateCustomerSettingsRequest,
    ) -> Result<CustomerSettings> {
        let url = self.url(&format!("/customers/{}/settings", customer_id));
        let response = self.http().put(&url).json(&req).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
