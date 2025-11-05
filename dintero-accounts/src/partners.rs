//! Module implementation.

use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingLink {
    pub id: String,
    pub url: String,
    pub partner_id: String,
    pub expires_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOnboardingLinkRequest {
    pub partner_id: String,
    pub redirect_url: Option<String>,
    pub prefill_data: Option<PrefillData>,
    pub expires_in_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefillData {
    pub organization_name: Option<String>,
    pub organization_number: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSignup {
    pub id: String,
    pub organization_name: String,
    pub organization_number: String,
    pub email: String,
    pub country: String,
    pub status: String,
    pub verification_status: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountSignupRequest {
    pub organization_name: String,
    pub organization_number: String,
    pub email: String,
    pub phone: Option<String>,
    pub country: String,
    pub prefill_data: Option<PrefillData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifySignupRequest {
    pub signup_id: String,
    pub verification_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantMatch {
    pub merchant_id: String,
    pub name: String,
    pub organization_number: Option<String>,
    pub match_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMerchantsRequest {
    pub query: String,
    pub country: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateMerchantRequest {
    pub merchant_id: String,
    pub reason: String,
    pub effective_date: Option<String>,
}

impl crate::client::AccountsClient {
    pub async fn create_onboarding_link(
        &self,
        request: &CreateOnboardingLinkRequest,
    ) -> Result<OnboardingLink> {
        self.execute_request(Method::POST, "partners/onboarding-links", Some(request)).await
    }

    pub async fn get_onboarding_link(&self, link_id: &str) -> Result<OnboardingLink> {
        self.execute_request(
            Method::GET,
            &format!("partners/onboarding-links/{}", link_id),
            None::<&()>,
        )
        .await
    }

    pub async fn create_account_signup(
        &self,
        request: &CreateAccountSignupRequest,
    ) -> Result<AccountSignup> {
        self.execute_request(Method::POST, "accounts/signup", Some(request)).await
    }

    pub async fn verify_signup(&self, request: &VerifySignupRequest) -> Result<AccountSignup> {
        self.execute_request(Method::POST, "accounts/signup/verify", Some(request)).await
    }

    pub async fn search_merchants(
        &self,
        request: &SearchMerchantsRequest,
    ) -> Result<Vec<MerchantMatch>> {
        self.execute_request(Method::POST, "partners/merchants/search", Some(request)).await
    }

    pub async fn terminate_merchant(&self, request: &TerminateMerchantRequest) -> Result<()> {
        self.execute_request(Method::POST, "partners/merchants/terminate", Some(request)).await
    }
}
