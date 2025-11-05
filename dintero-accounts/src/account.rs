use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDetails {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub organization_number: Option<String>,
    pub country: Option<String>,
    pub currency: Option<String>,
    pub settings: Option<AccountSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettings {
    pub auto_capture: Option<bool>,
    pub auto_capture_delay: Option<i32>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAccountRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub settings: Option<AccountSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePackage {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub pricing_model: Option<String>,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub account_id: String,
    pub asset_type: String,
    pub url: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAssetRequest {
    pub asset_type: String,
    pub data: Vec<u8>,
    pub metadata: Option<serde_json::Value>,
}

impl crate::client::AccountsClient {
    pub async fn get_account_details(&self) -> Result<AccountDetails> {
        self.execute_request(Method::GET, "accounts/current", None::<&()>)
            .await
    }

    pub async fn update_account_details(&self, request: &UpdateAccountRequest) -> Result<AccountDetails> {
        self.execute_request(Method::PUT, "accounts/current", Some(request))
            .await
    }

    pub async fn get_price_packages(&self) -> Result<Vec<PricePackage>> {
        self.execute_request(Method::GET, "accounts/current/price-packages", None::<&()>)
            .await
    }

    pub async fn list_assets(&self) -> Result<Vec<Asset>> {
        self.execute_request(Method::GET, "accounts/current/assets", None::<&()>)
            .await
    }

    pub async fn upload_asset(&self, request: &CreateAssetRequest) -> Result<Asset> {
        self.execute_request(Method::POST, "accounts/current/assets", Some(request))
            .await
    }

    pub async fn delete_asset(&self, asset_id: &str) -> Result<()> {
        self.execute_request(
            Method::DELETE,
            &format!("accounts/current/assets/{}", asset_id),
            None::<&()>,
        )
        .await
    }
}
