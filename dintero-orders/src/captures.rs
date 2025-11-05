use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capture {
    pub id: String,
    pub order_id: String,
    pub amount: i64,
    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CaptureItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureItem {
    pub line_id: String,
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCaptureRequest {
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CaptureItem>>,
}

impl CreateCaptureRequest {
    pub fn new(amount: i64) -> Self {
        Self {
            amount,
            items: None,
        }
    }

    pub fn with_items(mut self, items: Vec<CaptureItem>) -> Self {
        self.items = Some(items);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureListResponse {
    pub captures: Vec<Capture>,
}
