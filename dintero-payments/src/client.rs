//! Payments API client implementation.

use crate::fund_transfers::*;
use crate::payouts::*;
use crate::sellers::*;
use crate::settlements::*;
use crate::transactions::*;
use async_trait::async_trait;

pub type Result<T> = std::result::Result<T, PaymentsError>;

#[derive(Debug, thiserror::Error)]
pub enum PaymentsError {
    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Client error: {0}")]
    Client(String),
}

impl From<serde_json::Error> for PaymentsError {
    fn from(err: serde_json::Error) -> Self {
        PaymentsError::Serialization(err.to_string())
    }
}

#[async_trait]
pub trait PaymentsOperations: Send + Sync {
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

    async fn capture_transaction(
        &self,
        transaction_id: &str,
        request: CaptureTransactionRequest,
    ) -> Result<Transaction>;
    async fn refund_transaction(
        &self,
        transaction_id: &str,
        request: RefundTransactionRequest,
    ) -> Result<Transaction>;
    async fn void_transaction(
        &self,
        transaction_id: &str,
        request: VoidTransactionRequest,
    ) -> Result<Transaction>;
    async fn extend_authorization(
        &self,
        transaction_id: &str,
        request: ExtendAuthorizationRequest,
    ) -> Result<Transaction>;

    async fn list_settlements(&self) -> Result<SettlementListResponse>;
    async fn get_settlement_report_config(&self, config_id: &str)
        -> Result<SettlementReportConfig>;
    async fn list_settlement_report_configs(&self) -> Result<Vec<SettlementReportConfig>>;
    async fn create_settlement_report_config(
        &self,
        request: CreateSettlementReportConfigRequest,
    ) -> Result<SettlementReportConfig>;
    async fn update_settlement_report_config(
        &self,
        config_id: &str,
        request: UpdateSettlementReportConfigRequest,
    ) -> Result<SettlementReportConfig>;
    async fn delete_settlement_report_config(&self, config_id: &str) -> Result<()>;

    async fn list_payout_destinations(&self) -> Result<PayoutDestinationListResponse>;
    async fn create_payout_destination(
        &self,
        request: CreatePayoutDestinationRequest,
    ) -> Result<PayoutDestination>;
    async fn get_payout_balance(&self, destination_id: &str) -> Result<PayoutBalance>;
    async fn list_payout_transfers(
        &self,
        destination_id: &str,
    ) -> Result<PayoutTransferListResponse>;
    async fn create_payout_transfer(
        &self,
        request: CreatePayoutTransferRequest,
    ) -> Result<PayoutTransfer>;

    async fn initiate_fund_transfer(&self, request: FundTransferRequest) -> Result<FundTransfer>;
    async fn get_seller_balance(&self, destination_id: &str) -> Result<SellerBalance>;
    async fn list_seller_transfers(
        &self,
        destination_id: &str,
        params: ListSellerTransfersParams,
    ) -> Result<SellerTransfersResponse>;
}

#[async_trait]
pub trait PaymentsAdapter: Send + Sync {
    async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T>;
    async fn post_json<T: serde::de::DeserializeOwned, B: serde::Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T>;
    async fn put_json<T: serde::de::DeserializeOwned, B: serde::Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T>;
    async fn delete(&self, path: &str) -> Result<()>;
}

pub struct PaymentsClient<A: PaymentsAdapter> {
    adapter: A,
    account_id: String,
}

impl<A: PaymentsAdapter> PaymentsClient<A> {
    pub fn new(adapter: A, account_id: String) -> Self {
        Self { adapter, account_id }
    }
}

#[async_trait]
impl<A: PaymentsAdapter> PaymentsOperations for PaymentsClient<A> {
    async fn get_transaction(&self, transaction_id: &str) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}",
            self.account_id, transaction_id
        );
        self.adapter.get_json(&path).await
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
        if let Some(reference) = params.merchant_reference {
            query_params.push(format!("merchant_reference={}", reference));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.adapter.get_json(&path).await
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
        self.adapter.put_json(&path, &request).await
    }

    async fn capture_transaction(
        &self,
        transaction_id: &str,
        request: CaptureTransactionRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}/capture",
            self.account_id, transaction_id
        );
        self.adapter.post_json(&path, &request).await
    }

    async fn refund_transaction(
        &self,
        transaction_id: &str,
        request: RefundTransactionRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}/refund",
            self.account_id, transaction_id
        );
        self.adapter.post_json(&path, &request).await
    }

    async fn void_transaction(
        &self,
        transaction_id: &str,
        request: VoidTransactionRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}/void",
            self.account_id, transaction_id
        );
        self.adapter.post_json(&path, &request).await
    }

    async fn extend_authorization(
        &self,
        transaction_id: &str,
        request: ExtendAuthorizationRequest,
    ) -> Result<Transaction> {
        let path = format!(
            "accounts/{}/transactions/{}/extend_authorization",
            self.account_id, transaction_id
        );
        self.adapter.post_json(&path, &request).await
    }

    async fn list_settlements(&self) -> Result<SettlementListResponse> {
        let path = format!("accounts/{}/settlements", self.account_id);
        self.adapter.get_json(&path).await
    }

    async fn get_settlement_report_config(
        &self,
        config_id: &str,
    ) -> Result<SettlementReportConfig> {
        let path = format!(
            "accounts/{}/settlement_report_configs/{}",
            self.account_id, config_id
        );
        self.adapter.get_json(&path).await
    }

    async fn list_settlement_report_configs(&self) -> Result<Vec<SettlementReportConfig>> {
        let path = format!("accounts/{}/settlement_report_configs", self.account_id);
        self.adapter.get_json(&path).await
    }

    async fn create_settlement_report_config(
        &self,
        request: CreateSettlementReportConfigRequest,
    ) -> Result<SettlementReportConfig> {
        let path = format!("accounts/{}/settlement_report_configs", self.account_id);
        self.adapter.post_json(&path, &request).await
    }

    async fn update_settlement_report_config(
        &self,
        config_id: &str,
        request: UpdateSettlementReportConfigRequest,
    ) -> Result<SettlementReportConfig> {
        let path = format!(
            "accounts/{}/settlement_report_configs/{}",
            self.account_id, config_id
        );
        self.adapter.put_json(&path, &request).await
    }

    async fn delete_settlement_report_config(&self, config_id: &str) -> Result<()> {
        let path = format!(
            "accounts/{}/settlement_report_configs/{}",
            self.account_id, config_id
        );
        self.adapter.delete(&path).await
    }

    async fn list_payout_destinations(&self) -> Result<PayoutDestinationListResponse> {
        let path = format!(
            "accounts/{}/management/settings/approvals/payout_destinations",
            self.account_id
        );
        self.adapter.get_json(&path).await
    }

    async fn create_payout_destination(
        &self,
        request: CreatePayoutDestinationRequest,
    ) -> Result<PayoutDestination> {
        let path = format!(
            "accounts/{}/management/settings/approvals/payout_destinations",
            self.account_id
        );
        self.adapter.post_json(&path, &request).await
    }

    async fn get_payout_balance(&self, destination_id: &str) -> Result<PayoutBalance> {
        let path = format!(
            "accounts/{}/payout_destinations/{}/balance",
            self.account_id, destination_id
        );
        self.adapter.get_json(&path).await
    }

    async fn list_payout_transfers(
        &self,
        destination_id: &str,
    ) -> Result<PayoutTransferListResponse> {
        let path = format!(
            "accounts/{}/payout_destinations/{}/transfers",
            self.account_id, destination_id
        );
        self.adapter.get_json(&path).await
    }

    async fn create_payout_transfer(
        &self,
        request: CreatePayoutTransferRequest,
    ) -> Result<PayoutTransfer> {
        let path = format!("accounts/{}/payout/fund_transfers", self.account_id);
        self.adapter.post_json(&path, &request).await
    }

    async fn initiate_fund_transfer(&self, request: FundTransferRequest) -> Result<FundTransfer> {
        let path = format!("accounts/{}/payout/fund_transfers", self.account_id);
        self.adapter.post_json(&path, &request).await
    }

    async fn get_seller_balance(&self, destination_id: &str) -> Result<SellerBalance> {
        let path = format!(
            "accounts/{}/payout_destinations/{}/balance",
            self.account_id, destination_id
        );
        self.adapter.get_json(&path).await
    }

    async fn list_seller_transfers(
        &self,
        destination_id: &str,
        params: ListSellerTransfersParams,
    ) -> Result<SellerTransfersResponse> {
        let mut path = format!(
            "accounts/{}/payout_destinations/{}/transfers",
            self.account_id, destination_id
        );

        let mut query_params = Vec::new();
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(token) = params.page_token {
            query_params.push(format!("page_token={}", token));
        }
        if let Some(from_date) = params.from_date {
            query_params.push(format!("from_date={}", from_date));
        }
        if let Some(to_date) = params.to_date {
            query_params.push(format!("to_date={}", to_date));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.adapter.get_json(&path).await
    }
}
