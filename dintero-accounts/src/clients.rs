use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClient {
    pub client_id: String,
    pub client_name: String,
    pub client_secret: Option<String>,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<String>,
    pub response_types: Vec<String>,
    pub scope: Option<String>,
    pub token_endpoint_auth_method: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOAuthClientRequest {
    pub client_name: String,
    pub redirect_uris: Vec<String>,
    pub grant_types: Vec<String>,
    pub response_types: Vec<String>,
    pub scope: Option<String>,
    pub token_endpoint_auth_method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOAuthClientRequest {
    pub client_name: Option<String>,
    pub redirect_uris: Option<Vec<String>>,
    pub grant_types: Option<Vec<String>>,
    pub response_types: Option<Vec<String>>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientGrant {
    pub id: String,
    pub client_id: String,
    pub audience: String,
    pub scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClientGrantRequest {
    pub client_id: String,
    pub audience: String,
    pub scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCredential {
    pub id: String,
    pub name: String,
    pub client_id: String,
    pub scopes: Vec<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiCredentialRequest {
    pub name: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateClientSecretResponse {
    pub client_secret: String,
}

impl crate::client::AccountsClient {
    pub async fn list_oauth_clients(&self) -> Result<Vec<OAuthClient>> {
        self.execute_request(Method::GET, "oauth/clients", None::<&()>)
            .await
    }

    pub async fn get_oauth_client(&self, client_id: &str) -> Result<OAuthClient> {
        self.execute_request(
            Method::GET,
            &format!("oauth/clients/{}", client_id),
            None::<&()>,
        )
        .await
    }

    pub async fn create_oauth_client(
        &self,
        request: &CreateOAuthClientRequest,
    ) -> Result<OAuthClient> {
        self.execute_request(Method::POST, "oauth/clients", Some(request))
            .await
    }

    pub async fn update_oauth_client(
        &self,
        client_id: &str,
        request: &UpdateOAuthClientRequest,
    ) -> Result<OAuthClient> {
        self.execute_request(
            Method::PUT,
            &format!("oauth/clients/{}", client_id),
            Some(request),
        )
        .await
    }

    pub async fn delete_oauth_client(&self, client_id: &str) -> Result<()> {
        self.execute_request(
            Method::DELETE,
            &format!("oauth/clients/{}", client_id),
            None::<&()>,
        )
        .await
    }

    pub async fn rotate_client_secret(
        &self,
        client_id: &str,
    ) -> Result<RotateClientSecretResponse> {
        self.execute_request(
            Method::POST,
            &format!("oauth/clients/{}/secret", client_id),
            None::<&()>,
        )
        .await
    }

    pub async fn list_client_grants(&self) -> Result<Vec<ClientGrant>> {
        self.execute_request(Method::GET, "oauth/client-grants", None::<&()>)
            .await
    }

    pub async fn create_client_grant(&self, request: &CreateClientGrantRequest) -> Result<ClientGrant> {
        self.execute_request(Method::POST, "oauth/client-grants", Some(request))
            .await
    }

    pub async fn delete_client_grant(&self, grant_id: &str) -> Result<()> {
        self.execute_request(
            Method::DELETE,
            &format!("oauth/client-grants/{}", grant_id),
            None::<&()>,
        )
        .await
    }

    pub async fn list_api_credentials(&self) -> Result<Vec<ApiCredential>> {
        self.execute_request(Method::GET, "accounts/current/api-credentials", None::<&()>)
            .await
    }

    pub async fn create_api_credential(
        &self,
        request: &CreateApiCredentialRequest,
    ) -> Result<ApiCredential> {
        self.execute_request(
            Method::POST,
            "accounts/current/api-credentials",
            Some(request),
        )
        .await
    }

    pub async fn delete_api_credential(&self, credential_id: &str) -> Result<()> {
        self.execute_request(
            Method::DELETE,
            &format!("accounts/current/api-credentials/{}", credential_id),
            None::<&()>,
        )
        .await
    }
}
