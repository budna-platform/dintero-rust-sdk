//! Module implementation.

use crate::client::InsightsClient;
use crate::types::*;

pub struct KpisClient {
    pub(crate) client: InsightsClient,
}

impl KpisClient {
    pub async fn get_checkout_transaction_status(
        &self,
        params: KpiQueryParams,
    ) -> Result<CheckoutTransactionStatusKpiResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/accounts/{}/insight/kpi/checkout-transaction-status",
            self.client.base_url, self.client.account_id
        );

        let mut query_params = vec![
            ("from_date", params.from_date.to_rfc3339()),
            ("to_date", params.to_date.to_rfc3339()),
        ];

        if let Some(group_by) = &params.group_by {
            query_params.push(("group_by", group_by.clone()));
        }

        let response = self
            .client
            .http_client
            .get(&url)
            .query(&query_params)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn get_transactions(
        &self,
        params: KpiQueryParams,
    ) -> Result<TransactionKpiResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/accounts/{}/insight/kpi/transactions",
            self.client.base_url, self.client.account_id
        );

        let mut query_params = vec![
            ("from_date", params.from_date.to_rfc3339()),
            ("to_date", params.to_date.to_rfc3339()),
        ];

        if let Some(group_by) = &params.group_by {
            query_params.push(("group_by", group_by.clone()));
        }

        let response = self
            .client
            .http_client
            .get(&url)
            .query(&query_params)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn get_payment_methods(
        &self,
        params: KpiQueryParams,
    ) -> Result<PaymentMethodKpiResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/accounts/{}/insight/kpi/payment-methods",
            self.client.base_url, self.client.account_id
        );

        let mut query_params = vec![
            ("from_date", params.from_date.to_rfc3339()),
            ("to_date", params.to_date.to_rfc3339()),
        ];

        if let Some(group_by) = &params.group_by {
            query_params.push(("group_by", group_by.clone()));
        }

        let response = self
            .client
            .http_client
            .get(&url)
            .query(&query_params)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn get_revenue(
        &self,
        params: KpiQueryParams,
    ) -> Result<RevenueKpiResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/accounts/{}/insight/kpi/revenue",
            self.client.base_url, self.client.account_id
        );

        let mut query_params = vec![
            ("from_date", params.from_date.to_rfc3339()),
            ("to_date", params.to_date.to_rfc3339()),
        ];

        if let Some(group_by) = &params.group_by {
            query_params.push(("group_by", group_by.clone()));
        }

        let response = self
            .client
            .http_client
            .get(&url)
            .query(&query_params)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }
}
