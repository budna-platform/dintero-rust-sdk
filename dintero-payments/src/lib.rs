//! Copyright (c) 2024 Budna Marketplace AB
//! Author: Marcus Cvjeticanin
//!
//! Dintero Payments API client library.
//!
//! This crate provides types and clients for payment operations in the Dintero platform.

pub mod fund_transfers;
pub mod payouts;
pub mod sellers;
pub mod settlements;
pub mod transactions;

pub use fund_transfers::*;
pub use payouts::*;
pub use sellers::*;
pub use settlements::*;
pub use transactions::*;

mod client;
pub use client::*;
