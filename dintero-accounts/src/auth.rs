use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub code: Option<String>,
    pub redirect_uri: Option<String>,
    pub refresh_token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeTokenRequest {
    pub token: String,
    pub token_type_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordlessAuthRequest {
    pub email: String,
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordlessVerifyRequest {
    pub token: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaSetupRequest {
    pub mfa_type: String,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaSetupResponse {
    pub secret: Option<String>,
    pub qr_code: Option<String>,
    pub backup_codes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaVerifyRequest {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaChallengeRequest {
    pub session_token: String,
    pub mfa_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcConfiguration {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub jwks_uri: String,
    pub scopes_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryCode {
    pub code: String,
    pub used: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRecoveryCodesResponse {
    pub codes: Vec<String>,
}

impl crate::client::AccountsClient {
    pub async fn create_token(&self, request: &TokenRequest) -> Result<TokenResponse> {
        self.execute_request(Method::POST, "oauth/token", Some(request))
            .await
    }

    pub async fn revoke_token(&self, request: &RevokeTokenRequest) -> Result<()> {
        self.execute_request(Method::POST, "oauth/revoke", Some(request))
            .await
    }

    pub async fn initiate_passwordless_auth(
        &self,
        request: &PasswordlessAuthRequest,
    ) -> Result<()> {
        self.execute_request(Method::POST, "auth/passwordless/initiate", Some(request))
            .await
    }

    pub async fn verify_passwordless_auth(
        &self,
        request: &PasswordlessVerifyRequest,
    ) -> Result<TokenResponse> {
        self.execute_request(Method::POST, "auth/passwordless/verify", Some(request))
            .await
    }

    pub async fn setup_mfa(&self, request: &MfaSetupRequest) -> Result<MfaSetupResponse> {
        self.execute_request(Method::POST, "auth/mfa/setup", Some(request))
            .await
    }

    pub async fn verify_mfa(&self, request: &MfaVerifyRequest) -> Result<()> {
        self.execute_request(Method::POST, "auth/mfa/verify", Some(request))
            .await
    }

    pub async fn disable_mfa(&self) -> Result<()> {
        self.execute_request(Method::DELETE, "auth/mfa", None::<&()>)
            .await
    }

    pub async fn create_mfa_challenge(&self, request: &MfaChallengeRequest) -> Result<()> {
        self.execute_request(Method::POST, "auth/mfa/challenge", Some(request))
            .await
    }

    pub async fn get_oidc_configuration(&self) -> Result<OidcConfiguration> {
        self.execute_request(Method::GET, ".well-known/openid-configuration", None::<&()>)
            .await
    }

    pub async fn generate_recovery_codes(&self) -> Result<GenerateRecoveryCodesResponse> {
        self.execute_request(Method::POST, "auth/recovery-codes", None::<&()>)
            .await
    }

    pub async fn list_recovery_codes(&self) -> Result<Vec<RecoveryCode>> {
        self.execute_request(Method::GET, "auth/recovery-codes", None::<&()>)
            .await
    }
}
