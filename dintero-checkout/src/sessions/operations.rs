//! Module implementation.

use super::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct CreateSessionRequestPayload {
    pub url: SessionUrl,
    pub order: Order,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_terms_url: Option<String>,
}

impl From<CreateSessionRequest> for CreateSessionRequestPayload {
    fn from(req: CreateSessionRequest) -> Self {
        Self {
            url: req.url,
            order: req.order,
            profile_id: req.profile_id,
            return_url: req.return_url,
            merchant_terms_url: req.merchant_terms_url,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionListResponse {
    pub sessions: Vec<CheckoutSession>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ListSessionsParams {
    pub limit: Option<u32>,
    pub page_token: Option<String>,
}

impl ListSessionsParams {
    pub fn builder() -> ListSessionsParamsBuilder {
        ListSessionsParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ListSessionsParamsBuilder {
    limit: Option<u32>,
    page_token: Option<String>,
}

impl ListSessionsParamsBuilder {
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.page_token = Some(token.into());
        self
    }

    pub fn build(self) -> ListSessionsParams {
        ListSessionsParams {
            limit: self.limit,
            page_token: self.page_token,
        }
    }
}
