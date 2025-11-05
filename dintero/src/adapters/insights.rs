//! Module implementation.

use crate::DinteroClient;

#[cfg(feature = "insights")]
impl DinteroClient {
    pub fn insights(&self) -> dintero_insights::InsightsClient {
        dintero_insights::InsightsClient::new(
            std::sync::Arc::new(self.http.clone_inner()),
            self.http.base_url().to_string(),
            self.http.account_id().to_string(),
        )
    }
}
