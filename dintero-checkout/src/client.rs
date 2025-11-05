use crate::card_tokens::{CardToken, CardTokenListResponse, ListCardTokensParams};
use crate::sessions::{
    CheckoutSession, CreateProfileRequest, CreateSessionRequest, CreateSessionRequestPayload,
    ListSessionsParams, SessionListResponse, SessionProfile,
};
use crate::transactions::{
    CaptureRequest, ExtendAuthorizationRequest, ListTransactionsParams, RefundRequest, Transaction,
    TransactionListResponse, UpdateTransactionRequest, VoidRequest,
};
use async_trait::async_trait;

pub type Result<T> = std::result::Result<T, CheckoutError>;

#[derive(Debug, thiserror::Error)]
pub enum CheckoutError {
    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Client error: {0}")]
    Client(String),
}

impl From<serde_json::Error> for CheckoutError {
    fn from(err: serde_json::Error) -> Self {
        CheckoutError::Serialization(err.to_string())
    }
}

#[async_trait]
pub trait CheckoutOperations: Send + Sync {
    async fn create_session(&self, request: CreateSessionRequest) -> Result<CheckoutSession>;
    async fn get_session(&self, session_id: &str) -> Result<CheckoutSession>;
    async fn update_session(
        &self,
        session_id: &str,
        request: CreateSessionRequest,
    ) -> Result<CheckoutSession>;
    async fn list_sessions(&self, params: ListSessionsParams) -> Result<SessionListResponse>;
    async fn cancel_session(&self, session_id: &str) -> Result<CheckoutSession>;

    async fn create_profile(&self, request: CreateProfileRequest) -> Result<SessionProfile>;
    async fn get_profile(&self, profile_id: &str) -> Result<SessionProfile>;
    async fn update_profile(
        &self,
        profile_id: &str,
        request: CreateProfileRequest,
    ) -> Result<SessionProfile>;
    async fn delete_profile(&self, profile_id: &str) -> Result<()>;
    async fn list_profiles(&self) -> Result<Vec<SessionProfile>>;

    async fn get_transaction(&self, transaction_id: &str) -> Result<Transaction>;
    async fn list_transactions(
        &self,
        params: ListTransactionsParams,
    ) -> Result<TransactionListResponse>;
    async fn update_transaction(
        &self,
        transaction_id: &str,
        request: UpdateTransactionRequest,
    ) -> Result<Transaction>;
    async fn extend_authorization(&self, transaction_id: &str, days: u32) -> Result<Transaction>;
    async fn capture_transaction(
        &self,
        transaction_id: &str,
        request: CaptureRequest,
    ) -> Result<Transaction>;
    async fn refund_transaction(
        &self,
        transaction_id: &str,
        request: RefundRequest,
    ) -> Result<Transaction>;
    async fn void_transaction(
        &self,
        transaction_id: &str,
        request: VoidRequest,
    ) -> Result<Transaction>;

    async fn get_card_token(&self, token_id: &str) -> Result<CardToken>;
    async fn list_card_tokens(&self, params: ListCardTokensParams)
        -> Result<CardTokenListResponse>;
    async fn delete_card_token(&self, token_id: &str) -> Result<()>;
}

pub struct CheckoutClient<C> {
    client: C,
    account_id: String,
}

impl<C> CheckoutClient<C> {
    pub fn new(client: C, account_id: impl Into<String>) -> Self {
        Self {
            client,
            account_id: account_id.into(),
        }
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
}
