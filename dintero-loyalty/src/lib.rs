pub mod customers;
pub mod discounts;
pub mod products;
pub mod receipts;
pub mod wallets;
pub mod webhooks;
pub mod locations;
pub mod automations;
pub mod client;
pub mod types;
pub mod error;

pub use client::LoyaltyClient;
pub use error::{LoyaltyError, Result};
