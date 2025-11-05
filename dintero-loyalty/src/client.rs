//! Loyalty API client implementation.

use reqwest::Client;

pub struct LoyaltyClient {
    http: Client,
    base_url: String,
    account_id: String,
}

impl LoyaltyClient {
    pub fn new(http: Client, base_url: String, account_id: String) -> Self {
        Self { http, base_url, account_id }
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}/accounts/{}{}", self.base_url, self.account_id, path)
    }

    pub(crate) fn http(&self) -> &Client {
        &self.http
    }
}
