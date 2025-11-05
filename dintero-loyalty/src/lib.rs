//! Copyright (c) 2024 Budna Marketplace AB
//! Author: Marcus Cvjeticanin
//!
//! Dintero Loyalty API client library.
//!
//! This crate provides types and clients for managing loyalty programs in the Dintero platform.

pub mod automations;
pub mod client;
pub mod customers;
pub mod discounts;
pub mod error;
pub mod locations;
pub mod products;
pub mod receipts;
pub mod types;
pub mod wallets;
pub mod webhooks;

pub use client::LoyaltyClient;
pub use error::{LoyaltyError, Result};
