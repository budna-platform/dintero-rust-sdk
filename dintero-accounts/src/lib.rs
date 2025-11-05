//! Copyright (c) 2024 Budna Marketplace AB
//! Author: Marcus Cvjeticanin
//!
//! Dintero Accounts API client library.
//!
//! This crate provides types and clients for account management in the Dintero platform.

pub mod account;
pub mod auth;
pub mod client;
pub mod clients;
pub mod error;
pub mod gateways;
pub mod locations;
pub mod partners;
pub mod types;
pub mod users;

pub use client::AccountsClient;
pub use error::{AccountError, Result};
pub use types::*;
