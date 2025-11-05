//! Copyright (c) 2024 Budna Marketplace AB
//! Author: Marcus Cvjeticanin
//!
//! Dintero Insights API client library.
//!
//! This crate provides types and clients for analytics and insights in the Dintero platform.

pub mod client;
pub mod kpis;
pub mod report_configs;
pub mod reports;
pub mod types;

pub use client::InsightsClient;
pub use types::*;
