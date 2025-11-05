use crate::error::Result;
use crate::types::*;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

pub struct AccountsClient {
    client: Client,
    base_url: String,
    api_token: String,
}

impl AccountsClient {
    pub fn new(base_url: String, api_token: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_token,
        }
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let message = response.text().await.unwrap_or_default();
            Err(crate::error::AccountError::ApiError {
                status: status.as_u16(),
                message,
            })
        }
    }

    pub async fn get_account(&self, account_id: &str) -> Result<Account> {
        let url = format!("{}/v1/accounts/{}", self.base_url, account_id);
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn list_accounts(&self, page_token: Option<&str>) -> Result<AccountList> {
        let mut url = format!("{}/v1/accounts", self.base_url);
        if let Some(token) = page_token {
            url.push_str(&format!("?page_token={}", token));
        }

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn update_account(
        &self,
        account_id: &str,
        request: UpdateAccountRequest,
    ) -> Result<Account> {
        let url = format!("{}/v1/accounts/{}", self.base_url, account_id);
        let response = self
            .client
            .patch(&url)
            .bearer_auth(&self.api_token)
            .json(&request.build())
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn get_profile(&self, account_id: &str, profile_id: &str) -> Result<Profile> {
        let url = format!(
            "{}/v1/accounts/{}/profiles/{}",
            self.base_url, account_id, profile_id
        );
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn list_profiles(
        &self,
        account_id: &str,
        page_token: Option<&str>,
    ) -> Result<ProfileList> {
        let mut url = format!("{}/v1/accounts/{}/profiles", self.base_url, account_id);
        if let Some(token) = page_token {
            url.push_str(&format!("?page_token={}", token));
        }

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn update_profile(
        &self,
        account_id: &str,
        profile_id: &str,
        request: UpdateProfileRequest,
    ) -> Result<Profile> {
        let url = format!(
            "{}/v1/accounts/{}/profiles/{}",
            self.base_url, account_id, profile_id
        );
        let response = self
            .client
            .patch(&url)
            .bearer_auth(&self.api_token)
            .json(&request.build())
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn get_session(&self) -> Result<Session> {
        let url = format!("{}/v1/accounts/session", self.base_url);
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub(crate) async fn execute_request<T: DeserializeOwned, B: serde::Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        let url = format!("{}/v1/{}", self.base_url, path);
        let mut request = self.client.request(method, &url).bearer_auth(&self.api_token);

        if let Some(b) = body {
            request = request.json(b);
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }
}
