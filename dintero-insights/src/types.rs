//! Type definitions for insights and analytics.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfiguration {
    pub id: String,
    pub account_id: String,
    pub report_type: String,
    pub name: String,
    pub description: Option<String>,
    pub schedule: Option<ReportSchedule>,
    pub recipients: Vec<String>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSchedule {
    pub frequency: ScheduleFrequency,
    pub time: String,
    pub day_of_week: Option<u8>,
    pub day_of_month: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScheduleFrequency {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReportConfigurationRequest {
    pub report_type: String,
    pub name: String,
    pub description: Option<String>,
    pub schedule: Option<ReportSchedule>,
    pub recipients: Vec<String>,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReportConfigurationRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub schedule: Option<ReportSchedule>,
    pub recipients: Option<Vec<String>>,
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutTransactionStatusKpi {
    pub status: String,
    pub count: u64,
    pub amount: i64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutTransactionStatusKpiResponse {
    pub data: Vec<CheckoutTransactionStatusKpi>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionKpi {
    pub total_transactions: u64,
    pub total_amount: i64,
    pub currency: String,
    pub successful_transactions: u64,
    pub successful_amount: i64,
    pub failed_transactions: u64,
    pub refunded_transactions: u64,
    pub refunded_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionKpiResponse {
    pub data: TransactionKpi,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodKpi {
    pub payment_method: String,
    pub count: u64,
    pub amount: i64,
    pub currency: String,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodKpiResponse {
    pub data: Vec<PaymentMethodKpi>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueKpi {
    pub date: String,
    pub revenue: i64,
    pub currency: String,
    pub transaction_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueKpiResponse {
    pub data: Vec<RevenueKpi>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_revenue: i64,
    pub currency: String,
}

#[derive(Debug, Clone)]
pub struct KpiQueryParams {
    pub from_date: DateTime<Utc>,
    pub to_date: DateTime<Utc>,
    pub group_by: Option<String>,
}

impl KpiQueryParams {
    pub fn new(from_date: DateTime<Utc>, to_date: DateTime<Utc>) -> Self {
        Self { from_date, to_date, group_by: None }
    }

    pub fn with_group_by(mut self, group_by: String) -> Self {
        self.group_by = Some(group_by);
        self
    }
}
