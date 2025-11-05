use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConnection {
    pub id: String,
    pub gateway_type: String,
    pub name: String,
    pub enabled: bool,
    pub configuration: serde_json::Value,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BamboraConfiguration {
    pub merchant_number: String,
    pub access_token: Option<String>,
    pub secret_token: Option<String>,
    pub payfac: Option<bool>,
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VippsConfiguration {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub merchant_serial_number: String,
    pub subscription_key: Option<String>,
    pub psp: Option<bool>,
    pub fallback_url: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlarnaConfiguration {
    pub username: String,
    pub password: Option<String>,
    pub region: String,
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwishConfiguration {
    pub merchant_id: String,
    pub certificate: Option<String>,
    pub certificate_password: Option<String>,
    pub payfac: Option<bool>,
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectorConfiguration {
    pub username: String,
    pub password: Option<String>,
    pub store_id: String,
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayExConfiguration {
    pub merchant_id: String,
    pub token: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SantanderConfiguration {
    pub merchant_id: String,
    pub terminal_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstabankConfiguration {
    pub merchant_id: String,
    pub api_key: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KraviaConfiguration {
    pub merchant_id: String,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DinteroPspConfiguration {
    pub enabled: bool,
    pub merchant_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetaxeptConfiguration {
    pub merchant_id: String,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplePayConfiguration {
    pub merchant_id: String,
    pub certificate: Option<String>,
    pub certificate_password: Option<String>,
    pub domain_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisaConfiguration {
    pub merchant_id: String,
    pub api_key: Option<String>,
    pub tokenization_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MastercardConfiguration {
    pub merchant_id: String,
    pub api_key: Option<String>,
    pub tokenization_enabled: Option<bool>,
    pub three_ds_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGatewayRequest {
    pub gateway_type: String,
    pub name: String,
    pub configuration: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGatewayRequest {
    pub name: Option<String>,
    pub enabled: Option<bool>,
    pub configuration: Option<serde_json::Value>,
}

impl crate::client::AccountsClient {
    pub async fn list_gateway_connections(&self) -> Result<Vec<GatewayConnection>> {
        self.execute_request(
            Method::GET,
            "accounts/current/gateway-connections",
            None::<&()>,
        )
        .await
    }

    pub async fn get_gateway_connection(&self, gateway_id: &str) -> Result<GatewayConnection> {
        self.execute_request(
            Method::GET,
            &format!("accounts/current/gateway-connections/{}", gateway_id),
            None::<&()>,
        )
        .await
    }

    pub async fn create_gateway_connection(
        &self,
        request: &CreateGatewayRequest,
    ) -> Result<GatewayConnection> {
        self.execute_request(
            Method::POST,
            "accounts/current/gateway-connections",
            Some(request),
        )
        .await
    }

    pub async fn update_gateway_connection(
        &self,
        gateway_id: &str,
        request: &UpdateGatewayRequest,
    ) -> Result<GatewayConnection> {
        self.execute_request(
            Method::PUT,
            &format!("accounts/current/gateway-connections/{}", gateway_id),
            Some(request),
        )
        .await
    }

    pub async fn delete_gateway_connection(&self, gateway_id: &str) -> Result<()> {
        self.execute_request(
            Method::DELETE,
            &format!("accounts/current/gateway-connections/{}", gateway_id),
            None::<&()>,
        )
        .await
    }

    pub async fn create_bambora_connection(
        &self,
        name: String,
        config: BamboraConfiguration,
    ) -> Result<GatewayConnection> {
        let request = CreateGatewayRequest {
            gateway_type: "bambora".to_string(),
            name,
            configuration: serde_json::to_value(config).unwrap(),
        };
        self.create_gateway_connection(&request).await
    }

    pub async fn create_vipps_connection(
        &self,
        name: String,
        config: VippsConfiguration,
    ) -> Result<GatewayConnection> {
        let request = CreateGatewayRequest {
            gateway_type: "vipps".to_string(),
            name,
            configuration: serde_json::to_value(config).unwrap(),
        };
        self.create_gateway_connection(&request).await
    }

    pub async fn create_klarna_connection(
        &self,
        name: String,
        config: KlarnaConfiguration,
    ) -> Result<GatewayConnection> {
        let request = CreateGatewayRequest {
            gateway_type: "klarna".to_string(),
            name,
            configuration: serde_json::to_value(config).unwrap(),
        };
        self.create_gateway_connection(&request).await
    }

    pub async fn create_swish_connection(
        &self,
        name: String,
        config: SwishConfiguration,
    ) -> Result<GatewayConnection> {
        let request = CreateGatewayRequest {
            gateway_type: "swish".to_string(),
            name,
            configuration: serde_json::to_value(config).unwrap(),
        };
        self.create_gateway_connection(&request).await
    }

    pub async fn create_apple_pay_connection(
        &self,
        name: String,
        config: ApplePayConfiguration,
    ) -> Result<GatewayConnection> {
        let request = CreateGatewayRequest {
            gateway_type: "apple_pay".to_string(),
            name,
            configuration: serde_json::to_value(config).unwrap(),
        };
        self.create_gateway_connection(&request).await
    }
}
