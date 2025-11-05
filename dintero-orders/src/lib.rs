//! Copyright (c) 2024 Budna Marketplace AB
//! Author: Marcus Cvjeticanin
//!
//! Dintero Orders API client library.
//!
//! This crate provides types and clients for managing orders in the Dintero platform.

pub mod authorizations;
pub mod cancellations;
pub mod captures;
pub mod comments;
pub mod drafts;
pub mod events;
pub mod orders;
pub mod refunds;
pub mod sessions;

pub use authorizations::*;
pub use cancellations::*;
pub use captures::*;
pub use comments::*;
pub use drafts::*;
pub use events::*;
pub use orders::*;
pub use refunds::*;
pub use sessions::*;

mod client;
pub use client::*;
