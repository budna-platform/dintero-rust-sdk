//! Module implementation.

use crate::client::InsightsClient;
use crate::types::*;

pub struct ReportsClient {
    pub(crate) client: InsightsClient,
}

impl ReportsClient {
    pub async fn list_configurations(
        &self,
    ) -> Result<Vec<ReportConfiguration>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/accounts/{}/insight/reports/configuration",
            self.client.base_url, self.client.account_id
        );

        let response =
            self.client.http_client.get(&url).send().await?.error_for_status()?.json().await?;

        Ok(response)
    }

    pub async fn get_configuration(
        &self,
        configuration_id: &str,
    ) -> Result<ReportConfiguration, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/accounts/{}/insight/reports/configuration/{}",
            self.client.base_url, self.client.account_id, configuration_id
        );

        let response =
            self.client.http_client.get(&url).send().await?.error_for_status()?.json().await?;

        Ok(response)
    }

    pub async fn create_configuration(
        &self,
        request: CreateReportConfigurationRequest,
    ) -> Result<ReportConfiguration, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/accounts/{}/insight/reports/configuration",
            self.client.base_url, self.client.account_id
        );

        let response = self
            .client
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn update_configuration(
        &self,
        configuration_id: &str,
        request: UpdateReportConfigurationRequest,
    ) -> Result<ReportConfiguration, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/accounts/{}/insight/reports/configuration/{}",
            self.client.base_url, self.client.account_id, configuration_id
        );

        let response = self
            .client
            .http_client
            .put(&url)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn delete_configuration(
        &self,
        configuration_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/accounts/{}/insight/reports/configuration/{}",
            self.client.base_url, self.client.account_id, configuration_id
        );

        self.client.http_client.delete(&url).send().await?.error_for_status()?;

        Ok(())
    }
}
