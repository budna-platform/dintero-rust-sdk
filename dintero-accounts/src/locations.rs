use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: String,
    pub account_id: String,
    pub name: String,
    pub address: Option<Address>,
    pub contact: Option<Contact>,
    pub settings: Option<LocationSettings>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationSettings {
    pub receipt_email: Option<String>,
    pub receipt_sms: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLocationRequest {
    pub name: String,
    pub address: Option<Address>,
    pub contact: Option<Contact>,
    pub settings: Option<LocationSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLocationRequest {
    pub name: Option<String>,
    pub address: Option<Address>,
    pub contact: Option<Contact>,
    pub settings: Option<LocationSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terminal {
    pub id: String,
    pub location_id: String,
    pub name: String,
    pub terminal_type: String,
    pub serial_number: Option<String>,
    pub status: Option<String>,
    pub settings: Option<TerminalSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSettings {
    pub auto_print_receipt: Option<bool>,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTerminalRequest {
    pub name: String,
    pub terminal_type: String,
    pub serial_number: Option<String>,
    pub settings: Option<TerminalSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTerminalRequest {
    pub name: Option<String>,
    pub status: Option<String>,
    pub settings: Option<TerminalSettings>,
}

impl crate::client::AccountsClient {
    pub async fn list_locations(&self) -> Result<Vec<Location>> {
        self.execute_request(Method::GET, "accounts/current/locations", None::<&()>)
            .await
    }

    pub async fn get_location(&self, location_id: &str) -> Result<Location> {
        self.execute_request(
            Method::GET,
            &format!("accounts/current/locations/{}", location_id),
            None::<&()>,
        )
        .await
    }

    pub async fn create_location(&self, request: &CreateLocationRequest) -> Result<Location> {
        self.execute_request(Method::POST, "accounts/current/locations", Some(request))
            .await
    }

    pub async fn update_location(
        &self,
        location_id: &str,
        request: &UpdateLocationRequest,
    ) -> Result<Location> {
        self.execute_request(
            Method::PUT,
            &format!("accounts/current/locations/{}", location_id),
            Some(request),
        )
        .await
    }

    pub async fn delete_location(&self, location_id: &str) -> Result<()> {
        self.execute_request(
            Method::DELETE,
            &format!("accounts/current/locations/{}", location_id),
            None::<&()>,
        )
        .await
    }

    pub async fn list_terminals(&self, location_id: &str) -> Result<Vec<Terminal>> {
        self.execute_request(
            Method::GET,
            &format!("accounts/current/locations/{}/terminals", location_id),
            None::<&()>,
        )
        .await
    }

    pub async fn get_terminal(&self, location_id: &str, terminal_id: &str) -> Result<Terminal> {
        self.execute_request(
            Method::GET,
            &format!(
                "accounts/current/locations/{}/terminals/{}",
                location_id, terminal_id
            ),
            None::<&()>,
        )
        .await
    }

    pub async fn create_terminal(
        &self,
        location_id: &str,
        request: &CreateTerminalRequest,
    ) -> Result<Terminal> {
        self.execute_request(
            Method::POST,
            &format!("accounts/current/locations/{}/terminals", location_id),
            Some(request),
        )
        .await
    }

    pub async fn update_terminal(
        &self,
        location_id: &str,
        terminal_id: &str,
        request: &UpdateTerminalRequest,
    ) -> Result<Terminal> {
        self.execute_request(
            Method::PUT,
            &format!(
                "accounts/current/locations/{}/terminals/{}",
                location_id, terminal_id
            ),
            Some(request),
        )
        .await
    }

    pub async fn delete_terminal(&self, location_id: &str, terminal_id: &str) -> Result<()> {
        self.execute_request(
            Method::DELETE,
            &format!(
                "accounts/current/locations/{}/terminals/{}",
                location_id, terminal_id
            ),
            None::<&()>,
        )
        .await
    }
}
