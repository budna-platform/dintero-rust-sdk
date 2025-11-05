//! Insights API client implementation.

use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct InsightsClient {
    pub(crate) http_client: Arc<Client>,
    pub(crate) base_url: String,
    pub(crate) account_id: String,
}

impl InsightsClient {
    pub fn new(http_client: Arc<Client>, base_url: String, account_id: String) -> Self {
        Self { http_client, base_url, account_id }
    }

    pub fn kpis(&self) -> crate::kpis::KpisClient {
        crate::kpis::KpisClient { client: self.clone() }
    }

    pub fn reports(&self) -> crate::reports::ReportsClient {
        crate::reports::ReportsClient { client: self.clone() }
    }

    pub fn report_configs(&self) -> crate::report_configs::ReportConfigsClient {
        crate::report_configs::ReportConfigsClient { client: self.clone() }
    }
}
