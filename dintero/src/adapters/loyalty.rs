//! Loyalty API adapter.

use crate::DinteroClient;

#[cfg(feature = "loyalty")]
pub struct LoyaltyAdapter {
    client: dintero_loyalty::LoyaltyClient,
}

#[cfg(feature = "loyalty")]
impl LoyaltyAdapter {
    pub(crate) fn new(dintero_client: &DinteroClient) -> Self {
        let http_client = dintero_client.http_client();
        let loyalty_client = dintero_loyalty::LoyaltyClient::new(
            http_client.clone_inner(),
            http_client.base_url().to_string(),
            http_client.account_id().to_string(),
        );

        Self { client: loyalty_client }
    }

    pub fn customers(&self) -> &dintero_loyalty::LoyaltyClient {
        &self.client
    }

    pub fn discounts(&self) -> &dintero_loyalty::LoyaltyClient {
        &self.client
    }

    pub fn products(&self) -> &dintero_loyalty::LoyaltyClient {
        &self.client
    }

    pub fn receipts(&self) -> &dintero_loyalty::LoyaltyClient {
        &self.client
    }

    pub fn wallets(&self) -> &dintero_loyalty::LoyaltyClient {
        &self.client
    }

    pub fn webhooks(&self) -> &dintero_loyalty::LoyaltyClient {
        &self.client
    }

    pub fn locations(&self) -> &dintero_loyalty::LoyaltyClient {
        &self.client
    }

    pub fn automations(&self) -> &dintero_loyalty::LoyaltyClient {
        &self.client
    }
}
