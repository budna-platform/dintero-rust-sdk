//! Module implementation.

use super::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardTokenListResponse {
    pub tokens: Vec<CardToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ListCardTokensParams {
    pub limit: Option<u32>,
    pub page_token: Option<String>,
    pub status: Option<CardTokenStatus>,
}

impl ListCardTokensParams {
    pub fn builder() -> ListCardTokensParamsBuilder {
        ListCardTokensParamsBuilder::default()
    }
}

#[derive(Default)]
pub struct ListCardTokensParamsBuilder {
    limit: Option<u32>,
    page_token: Option<String>,
    status: Option<CardTokenStatus>,
}

impl ListCardTokensParamsBuilder {
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.page_token = Some(token.into());
        self
    }

    pub fn status(mut self, status: CardTokenStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn build(self) -> ListCardTokensParams {
        ListCardTokensParams {
            limit: self.limit,
            page_token: self.page_token,
            status: self.status,
        }
    }
}
