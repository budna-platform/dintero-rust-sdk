//! Location management.

use crate::client::LoyaltyClient;
use crate::error::Result;
use crate::types::{Address, PaginatedResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: Option<String>,
    pub address: Option<Address>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub opening_hours: Option<serde_json::Value>,
    pub active: bool,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLocationRequest {
    pub name: String,
    pub description: Option<String>,
    pub address: Option<Address>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub opening_hours: Option<serde_json::Value>,
    pub active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLocationRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub address: Option<Address>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub opening_hours: Option<serde_json::Value>,
    pub active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
pub struct ListLocationsRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub active: Option<bool>,
}

impl LoyaltyClient {
    pub async fn create_location(&self, req: CreateLocationRequest) -> Result<Location> {
        let url = self.url("/locations");
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

    pub async fn get_location(&self, location_id: &Uuid) -> Result<Location> {
        let url = self.url(&format!("/locations/{}", location_id));
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

    pub async fn update_location(
        &self,
        location_id: &Uuid,
        req: UpdateLocationRequest,
    ) -> Result<Location> {
        let url = self.url(&format!("/locations/{}", location_id));
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

    pub async fn delete_location(&self, location_id: &Uuid) -> Result<()> {
        let url = self.url(&format!("/locations/{}", location_id));
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

    pub async fn list_locations(
        &self,
        req: ListLocationsRequest,
    ) -> Result<PaginatedResponse<Location>> {
        let mut url = self.url("/locations");
        let mut params = vec![];

        if let Some(limit) = req.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = req.offset {
            params.push(format!("offset={}", offset));
        }
        if let Some(active) = req.active {
            params.push(format!("active={}", active));
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
}
