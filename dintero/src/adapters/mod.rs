#[cfg(feature = "checkout")]
pub mod checkout;

#[cfg(feature = "checkout")]
pub use checkout::CheckoutHttpAdapter;

#[cfg(feature = "orders")]
pub mod orders;

#[cfg(feature = "payments")]
pub mod payments;

#[cfg(feature = "accounts")]
pub mod accounts;

#[cfg(feature = "loyalty")]
pub mod loyalty;

#[cfg(feature = "loyalty")]
pub use loyalty::LoyaltyAdapter;
