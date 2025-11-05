//! Module implementation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub id: String,
    pub account_id: String,
    pub currency: String,
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_from: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_to: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_date: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementListResponse {
    pub settlements: Vec<Settlement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementReportConfig {
    pub id: String,
    pub account_id: String,
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateSettlementReportConfigRequest {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<String>,
}

impl CreateSettlementReportConfigRequest {
    pub fn new() -> Self {
        Self {
            enabled: true,
            email: None,
            file_format: None,
        }
    }

    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn with_file_format(mut self, format: impl Into<String>) -> Self {
        self.file_format = Some(format.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Default for CreateSettlementReportConfigRequest {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateSettlementReportConfigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<String>,
}

impl UpdateSettlementReportConfigRequest {
    pub fn new() -> Self {
        Self {
            enabled: None,
            email: None,
            file_format: None,
        }
    }

    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn with_file_format(mut self, format: impl Into<String>) -> Self {
        self.file_format = Some(format.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
}

impl Default for UpdateSettlementReportConfigRequest {
    fn default() -> Self {
        Self::new()
    }
}
