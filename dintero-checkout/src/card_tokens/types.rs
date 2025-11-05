use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardTokenStatus {
    Active,
    Expired,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardToken {
    pub id: String,
    pub status: CardTokenStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_pan: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_month: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_year: Option<String>,
}
