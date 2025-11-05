//! Module implementation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pagination<T> {
    pub data: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl<T> Pagination<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            starting_after: None,
            has_more: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> Default for Pagination<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            starting_after: None,
            has_more: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaginationParams {
    pub limit: Option<u32>,
    pub starting_after: Option<String>,
}

impl PaginationParams {
    pub fn builder() -> PaginationParamsBuilder {
        PaginationParamsBuilder::default()
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self { limit: Some(50), starting_after: None }
    }
}

#[derive(Default)]
pub struct PaginationParamsBuilder {
    limit: Option<u32>,
    starting_after: Option<String>,
}

impl PaginationParamsBuilder {
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn starting_after(mut self, cursor: impl Into<String>) -> Self {
        self.starting_after = Some(cursor.into());
        self
    }

    pub fn build(self) -> PaginationParams {
        PaginationParams {
            limit: self.limit,
            starting_after: self.starting_after,
        }
    }
}
