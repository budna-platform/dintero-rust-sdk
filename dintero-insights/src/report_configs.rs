//! Module implementation.

use crate::types::*;

pub struct ReportConfigsClient {
    pub(crate) client: crate::client::InsightsClient,
}

impl ReportConfigsClient {
    pub async fn create(
        &self,
        request: CreateReportConfigurationRequest,
    ) -> Result<ReportConfiguration, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/accounts/{}/reports/configuration",
            self.client.base_url, self.client.account_id
        );

        let response = self.client.http_client.post(&url).json(&request).send().await?;

        let config = response.json().await?;
        Ok(config)
    }

    pub async fn get(
        &self,
        config_id: &str,
    ) -> Result<ReportConfiguration, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/accounts/{}/reports/configuration/{}",
            self.client.base_url, self.client.account_id, config_id
        );

        let response = self.client.http_client.get(&url).send().await?;
        let config = response.json().await?;
        Ok(config)
    }

    pub async fn list(&self) -> Result<Vec<ReportConfiguration>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/accounts/{}/reports/configuration",
            self.client.base_url, self.client.account_id
        );

        let response = self.client.http_client.get(&url).send().await?;
        let configs = response.json().await?;
        Ok(configs)
    }

    pub async fn update(
        &self,
        config_id: &str,
        request: UpdateReportConfigurationRequest,
    ) -> Result<ReportConfiguration, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/accounts/{}/reports/configuration/{}",
            self.client.base_url, self.client.account_id, config_id
        );

        let response = self.client.http_client.put(&url).json(&request).send().await?;

        let config = response.json().await?;
        Ok(config)
    }

    pub async fn delete(&self, config_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "{}/accounts/{}/reports/configuration/{}",
            self.client.base_url, self.client.account_id, config_id
        );

        self.client.http_client.delete(&url).send().await?;
        Ok(())
    }
}
