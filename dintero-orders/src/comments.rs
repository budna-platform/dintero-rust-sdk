//! Module implementation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub order_id: String,
    pub text: String,
    pub created_by: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCommentRequest {
    pub text: String,
}

impl CreateCommentRequest {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}
