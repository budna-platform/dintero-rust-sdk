//! Checkout API client implementation.
//!
//! This module provides the main client for interacting with the Dintero Checkout API.
//! The client supports session management, payment processing, transaction operations,
//! and more.

use crate::api_keys::{ApiKey, CreateApiKeyRequest, CreateApiKeyResponse, RotateApiKeyResponse};
use crate::card_tokens::{CardToken, CardTokenListResponse, ListCardTokensParams};
use crate::credit_checks::{CreditCheckRequest, CreditCheckResponse};
use crate::qr_codes::{QrCodeRequest, QrCodeResponse};
use crate::secrets::{CreateSignatureSecretRequest, SignatureSecret};
use crate::sessions::{
    CheckoutSession, CreateProfileRequest, CreateSessionRequest, CreateSessionRequestPayload,
    ListSessionsParams, SessionListResponse, SessionProfile,
};
use crate::transactions::{
    CaptureRequest, ExtendAuthorizationRequest, ListTransactionsParams, RefundRequest, Transaction,
    TransactionListResponse, UpdateTransactionRequest, VoidRequest,
};
use async_trait::async_trait;

/// Result type for checkout operations.
pub type Result<T> = std::result::Result<T, CheckoutError>;

/// Errors that can occur during checkout operations.
#[derive(Debug, thiserror::Error)]
pub enum CheckoutError {
    /// Serialization or deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// General client error.
    #[error("Client error: {0}")]
    Client(String),
}

impl From<serde_json::Error> for CheckoutError {
    fn from(err: serde_json::Error) -> Self {
        CheckoutError::Serialization(err.to_string())
    }
}

/// Trait defining all checkout operations.
///
/// This trait is implemented by the checkout client and defines all available
/// operations for managing checkout sessions, transactions, and related resources.
#[async_trait]
pub trait CheckoutOperations: Send + Sync {
    /// Creates a new checkout session.
    async fn create_session(&self, request: CreateSessionRequest) -> Result<CheckoutSession>;

    /// Retrieves a checkout session by ID.
    async fn get_session(&self, session_id: &str) -> Result<CheckoutSession>;

    /// Updates an existing checkout session.
    async fn update_session(
        &self,
        session_id: &str,
        request: CreateSessionRequest,
    ) -> Result<CheckoutSession>;

    /// Lists checkout sessions with optional filtering.
    async fn list_sessions(&self, params: ListSessionsParams) -> Result<SessionListResponse>;

    /// Cancels a checkout session.
    async fn cancel_session(&self, session_id: &str) -> Result<CheckoutSession>;

    /// Creates a new session profile.
    async fn create_profile(&self, request: CreateProfileRequest) -> Result<SessionProfile>;

    /// Retrieves a session profile by ID.
    async fn get_profile(&self, profile_id: &str) -> Result<SessionProfile>;

    /// Updates an existing session profile.
    async fn update_profile(
        &self,
        profile_id: &str,
        request: CreateProfileRequest,
    ) -> Result<SessionProfile>;

    /// Deletes a session profile.
    async fn delete_profile(&self, profile_id: &str) -> Result<()>;

    /// Lists all session profiles.
    async fn list_profiles(&self) -> Result<Vec<SessionProfile>>;

    /// Retrieves a transaction by ID.
    async fn get_transaction(&self, transaction_id: &str) -> Result<Transaction>;

    /// Lists transactions with optional filtering.
    async fn list_transactions(
        &self,
        params: ListTransactionsParams,
    ) -> Result<TransactionListResponse>;

    /// Updates a transaction.
    async fn update_transaction(
        &self,
        transaction_id: &str,
        request: UpdateTransactionRequest,
    ) -> Result<Transaction>;

    /// Extends the authorization period for a transaction.
    async fn extend_authorization(&self, transaction_id: &str, days: u32) -> Result<Transaction>;

    /// Captures funds from an authorized transaction.
    async fn capture_transaction(
        &self,
        transaction_id: &str,
        request: CaptureRequest,
    ) -> Result<Transaction>;

    /// Refunds a captured transaction.
    async fn refund_transaction(
        &self,
        transaction_id: &str,
        request: RefundRequest,
    ) -> Result<Transaction>;

    /// Voids an authorized transaction.
    async fn void_transaction(
        &self,
        transaction_id: &str,
        request: VoidRequest,
    ) -> Result<Transaction>;

    /// Retrieves a card token by ID.
    async fn get_card_token(&self, token_id: &str) -> Result<CardToken>;

    /// Lists card tokens with optional filtering.
    async fn list_card_tokens(&self, params: ListCardTokensParams)
        -> Result<CardTokenListResponse>;

    /// Deletes a card token.
    async fn delete_card_token(&self, token_id: &str) -> Result<()>;

    /// Creates a new API key.
    async fn create_api_key(&self, request: CreateApiKeyRequest) -> Result<CreateApiKeyResponse>;

    /// Lists all API keys.
    async fn list_api_keys(&self) -> Result<Vec<ApiKey>>;

    /// Deletes an API key.
    async fn delete_api_key(&self, api_key_id: &str) -> Result<()>;

    /// Rotates an API key.
    async fn rotate_api_key(&self, api_key_id: &str) -> Result<RotateApiKeyResponse>;

    /// Creates a signature secret for webhook validation.
    async fn create_signature_secret(
        &self,
        request: CreateSignatureSecretRequest,
    ) -> Result<SignatureSecret>;

    /// Retrieves the current signature secret.
    async fn get_signature_secret(&self) -> Result<SignatureSecret>;

    /// Generates a QR code for a checkout session.
    async fn generate_qr_code(&self, request: QrCodeRequest) -> Result<QrCodeResponse>;

    /// Performs a credit check for a customer.
    async fn perform_credit_check(
        &self,
        request: CreditCheckRequest,
    ) -> Result<CreditCheckResponse>;
}

pub struct CheckoutClient<C> {
    client: C,
    account_id: String,
}

impl<C> CheckoutClient<C> {
    pub fn new(client: C, account_id: impl Into<String>) -> Self {
        Self { client, account_id: account_id.into() }
    }
}

#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn post_json<B: serde::Serialize + Send + Sync, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T>;

    async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T>;

    async fn put_json<B: serde::Serialize + Send + Sync, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T>;

    async fn delete(&self, path: &str) -> Result<()>;
}

#[async_trait]
impl<C: HttpClient> CheckoutOperations for CheckoutClient<C> {
    async fn create_session(&self, request: CreateSessionRequest) -> Result<CheckoutSession> {
        let path = format!("accounts/{}/sessions", self.account_id);
        let payload: CreateSessionRequestPayload = request.into();
        self.client.post_json(&path, &payload).await
    }

    async fn get_session(&self, session_id: &str) -> Result<CheckoutSession> {
        let path = format!("accounts/{}/sessions/{}", self.account_id, session_id);
        self.client.get_json(&path).await
    }

    async fn update_session(
        &self,
        session_id: &str,
        request: CreateSessionRequest,
    ) -> Result<CheckoutSession> {
        let path = format!("accounts/{}/sessions/{}", self.account_id, session_id);
        let payload: CreateSessionRequestPayload = request.into();
        self.client.put_json(&path, &payload).await
    }

    async fn list_sessions(&self, params: ListSessionsParams) -> Result<SessionListResponse> {
        let mut path = format!("accounts/{}/sessions", self.account_id);

        let mut query_params = Vec::new();
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(token) = params.page_token {
            query_params.push(format!("page_token={}", token));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.client.get_json(&path).await
    }

    async fn cancel_session(&self, session_id: &str) -> Result<CheckoutSession> {
        let path = format!(
            "accounts/{}/sessions/{}/cancel",
            self.account_id, session_id
        );
        self.client.post_json(&path, &serde_json::json!({})).await
    }

    async fn create_profile(&self, request: CreateProfileRequest) -> Result<SessionProfile> {
        let path = format!("accounts/{}/session_profile", self.account_id);
        self.client.post_json(&path, &request).await
    }

    async fn get_profile(&self, profile_id: &str) -> Result<SessionProfile> {
        let path = format!(
            "accounts/{}/session_profile/{}",
            self.account_id, profile_id
        );
        self.client.get_json(&path).await
    }

    async fn update_profile(
        &self,
        profile_id: &str,
        request: CreateProfileRequest,
    ) -> Result<SessionProfile> {
        let path = format!(
            "accounts/{}/session_profile/{}",
            self.account_id, profile_id
        );
        self.client.put_json(&path, &request).await
    }

    async fn delete_profile(&self, profile_id: &str) -> Result<()> {
        let path = format!(
            "accounts/{}/session_profile/{}",
            self.account_id, profile_id
        );
        self.client.delete(&path).await
    }

    async fn list_profiles(&self) -> Result<Vec<SessionProfile>> {
        let path = format!("accounts/{}/session_profile", self.account_id);
        self.client.get_json(&path).await
    }

    async fn get_transaction(&self, transaction_id: &str) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}",
            self.account_id, transaction_id
        );
        self.client.get_json(&path).await
    }

    async fn list_transactions(
        &self,
        params: ListTransactionsParams,
    ) -> Result<TransactionListResponse> {
        let mut path = format!("accounts/{}/transactions", self.account_id);

        let mut query_params = Vec::new();
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(token) = params.page_token {
            query_params.push(format!("page_token={}", token));
        }
        if let Some(status) = params.status {
            query_params.push(format!("status={:?}", status));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.client.get_json(&path).await
    }

    async fn update_transaction(
        &self,
        transaction_id: &str,
        request: UpdateTransactionRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}",
            self.account_id, transaction_id
        );
        self.client.put_json(&path, &request).await
    }

    async fn extend_authorization(&self, transaction_id: &str, days: u32) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}/extend_authorization",
            self.account_id, transaction_id
        );
        let request = ExtendAuthorizationRequest::new(days);
        self.client.post_json(&path, &request).await
    }

    async fn capture_transaction(
        &self,
        transaction_id: &str,
        request: CaptureRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}/capture",
            self.account_id, transaction_id
        );
        self.client.post_json(&path, &request).await
    }

    async fn refund_transaction(
        &self,
        transaction_id: &str,
        request: RefundRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}/refund",
            self.account_id, transaction_id
        );
        self.client.post_json(&path, &request).await
    }

    async fn void_transaction(
        &self,
        transaction_id: &str,
        request: VoidRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}/void",
            self.account_id, transaction_id
        );
        self.client.post_json(&path, &request).await
    }

    async fn get_card_token(&self, token_id: &str) -> Result<CardToken> {
        let path = format!("accounts/{}/card-tokens/{}", self.account_id, token_id);
        self.client.get_json(&path).await
    }

    async fn list_card_tokens(
        &self,
        params: ListCardTokensParams,
    ) -> Result<CardTokenListResponse> {
        let mut path = format!("accounts/{}/card-tokens", self.account_id);

        let mut query_params = Vec::new();
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(token) = params.page_token {
            query_params.push(format!("page_token={}", token));
        }
        if let Some(status) = params.status {
            query_params.push(format!("status={:?}", status));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.client.get_json(&path).await
    }

    async fn delete_card_token(&self, token_id: &str) -> Result<()> {
        let path = format!("accounts/{}/card-tokens/{}", self.account_id, token_id);
        self.client.delete(&path).await
    }

    async fn create_api_key(&self, request: CreateApiKeyRequest) -> Result<CreateApiKeyResponse> {
        let path = format!("accounts/{}/api_keys", self.account_id);
        self.client.post_json(&path, &request).await
    }

    async fn list_api_keys(&self) -> Result<Vec<ApiKey>> {
        let path = format!("accounts/{}/api_keys", self.account_id);
        self.client.get_json(&path).await
    }

    async fn delete_api_key(&self, api_key_id: &str) -> Result<()> {
        let path = format!("accounts/{}/api_keys/{}", self.account_id, api_key_id);
        self.client.delete(&path).await
    }

    async fn rotate_api_key(&self, api_key_id: &str) -> Result<RotateApiKeyResponse> {
        let path = format!(
            "accounts/{}/api_keys/{}/rotate",
            self.account_id, api_key_id
        );
        self.client.post_json(&path, &serde_json::json!({})).await
    }

    async fn create_signature_secret(
        &self,
        request: CreateSignatureSecretRequest,
    ) -> Result<SignatureSecret> {
        let path = format!("accounts/{}/signature", self.account_id);
        self.client.post_json(&path, &request).await
    }

    async fn get_signature_secret(&self) -> Result<SignatureSecret> {
        let path = format!("accounts/{}/signature", self.account_id);
        self.client.get_json(&path).await
    }

    async fn generate_qr_code(&self, request: QrCodeRequest) -> Result<QrCodeResponse> {
        let path = format!(
            "accounts/{}/sessions/{}/qr",
            self.account_id, request.session_id
        );
        self.client.post_json(&path, &request).await
    }

    async fn perform_credit_check(
        &self,
        request: CreditCheckRequest,
    ) -> Result<CreditCheckResponse> {
        let path = "creditchecks".to_string();
        self.client.post_json(&path, &request).await
    }
}
