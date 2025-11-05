//! Accounts API adapter.

use crate::client::HttpClient;

#[cfg(feature = "accounts")]
use dintero_accounts::{
    account::AccountDetails,
    clients::OAuthClient,
    gateways::GatewayConnection,
    locations::{Location, Terminal},
    users::User,
    Account, AccountList, AccountsClient, Profile, ProfileList, Session, UpdateAccountRequest,
    UpdateProfileRequest,
};

#[cfg(feature = "accounts")]
pub struct AccountsAdapter {
    accounts_client: AccountsClient,
}

#[cfg(feature = "accounts")]
impl AccountsAdapter {
    pub fn new(client: &HttpClient) -> Self {
        let accounts_client =
            AccountsClient::new(client.base_url.clone(), client.api_token.clone());
        Self { accounts_client }
    }

    pub async fn get_account(
        &self,
        account_id: &str,
    ) -> Result<Account, dintero_accounts::AccountError> {
        self.accounts_client.get_account(account_id).await
    }

    pub async fn list_accounts(
        &self,
        page_token: Option<&str>,
    ) -> Result<AccountList, dintero_accounts::AccountError> {
        self.accounts_client.list_accounts(page_token).await
    }

    pub async fn update_account(
        &self,
        account_id: &str,
        request: UpdateAccountRequest,
    ) -> Result<Account, dintero_accounts::AccountError> {
        self.accounts_client.update_account(account_id, request).await
    }

    pub async fn get_profile(
        &self,
        account_id: &str,
        profile_id: &str,
    ) -> Result<Profile, dintero_accounts::AccountError> {
        self.accounts_client.get_profile(account_id, profile_id).await
    }

    pub async fn list_profiles(
        &self,
        account_id: &str,
        page_token: Option<&str>,
    ) -> Result<ProfileList, dintero_accounts::AccountError> {
        self.accounts_client.list_profiles(account_id, page_token).await
    }

    pub async fn update_profile(
        &self,
        account_id: &str,
        profile_id: &str,
        request: UpdateProfileRequest,
    ) -> Result<Profile, dintero_accounts::AccountError> {
        self.accounts_client.update_profile(account_id, profile_id, request).await
    }

    pub async fn get_session(&self) -> Result<Session, dintero_accounts::AccountError> {
        self.accounts_client.get_session().await
    }

    pub async fn get_account_details(
        &self,
    ) -> Result<AccountDetails, dintero_accounts::AccountError> {
        self.accounts_client.get_account_details().await
    }

    pub async fn list_locations(&self) -> Result<Vec<Location>, dintero_accounts::AccountError> {
        self.accounts_client.list_locations().await
    }

    pub async fn get_location(
        &self,
        location_id: &str,
    ) -> Result<Location, dintero_accounts::AccountError> {
        self.accounts_client.get_location(location_id).await
    }

    pub async fn list_terminals(
        &self,
        location_id: &str,
    ) -> Result<Vec<Terminal>, dintero_accounts::AccountError> {
        self.accounts_client.list_terminals(location_id).await
    }

    pub async fn list_users(&self) -> Result<Vec<User>, dintero_accounts::AccountError> {
        self.accounts_client.list_users().await
    }

    pub async fn get_user(&self, user_id: &str) -> Result<User, dintero_accounts::AccountError> {
        self.accounts_client.get_user(user_id).await
    }

    pub async fn list_oauth_clients(
        &self,
    ) -> Result<Vec<OAuthClient>, dintero_accounts::AccountError> {
        self.accounts_client.list_oauth_clients().await
    }

    pub async fn list_gateway_connections(
        &self,
    ) -> Result<Vec<GatewayConnection>, dintero_accounts::AccountError> {
        self.accounts_client.list_gateway_connections().await
    }
}
