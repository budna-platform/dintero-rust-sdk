# Dintero Rust SDK

[![Crates.io](https://img.shields.io/crates/v/dintero.svg)](https://crates.io/crates/dintero)
[![Documentation](https://docs.rs/dintero/badge.svg)](https://docs.rs/dintero)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive Rust SDK for the [Dintero API](https://docs.dintero.com), providing convenient and type-safe access to all Dintero services.

## Features

- **Full API Coverage**: Complete implementation of all Dintero APIs
- **Type-Safe**: Strongly typed request and response models
- **Async/Await**: Built on `tokio` and `reqwest` for async operations
- **Modular**: Enable only the features you need
- **Builder Pattern**: Ergonomic API design with builder patterns
- **Flexible Authentication**: Support for various authentication methods

### Available Modules

- **Checkout**: Payment sessions, card tokens, and transactions
- **Orders**: Order management and operations
- **Payments**: Payment operations and captures
- **Accounts**: Account management and profiles
- **Loyalty**: Loyalty programs and rewards
- **Insights**: Analytics and reporting

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dintero = "0.1"
```

### Feature Flags

By default, all features are enabled. You can selectively enable only what you need:

```toml
[dependencies]
dintero = { version = "0.1", default-features = false, features = ["checkout", "orders"] }
```

Available features:
- `checkout` - Checkout API support
- `orders` - Orders API support
- `payments` - Payments API support
- `accounts` - Accounts API support
- `loyalty` - Loyalty API support
- `insights` - Insights API support

## Quick Start

### Basic Usage

```rust
use dintero::{DinteroClient, DinteroConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let config = DinteroConfig::new(
        "your-account-id",
        "your-client-id",
        "your-client-secret"
    );
    
    let client = DinteroClient::new(config);
    
    Ok(())
}
```

### Using Environment Variables

```rust
use dintero::DinteroClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reads from DINTERO_ACCOUNT_ID, DINTERO_CLIENT_ID, DINTERO_CLIENT_SECRET
    let client = DinteroClient::from_env()?;
    
    Ok(())
}
```

### Creating a Checkout Session

```rust
use dintero::{DinteroClient, DinteroConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DinteroClient::from_env()?;
    
    #[cfg(feature = "checkout")]
    {
        use dintero_checkout::{SessionCreateRequest, Order, OrderLine, Money};
        
        let request = SessionCreateRequest::builder()
            .url("https://example.com/return".to_string())
            .order(
                Order::builder()
                    .amount(Money::new(10000, "NOK"))
                    .add_item(
                        OrderLine::builder()
                            .id("item-1".to_string())
                            .description("Test Item".to_string())
                            .quantity(1)
                            .amount(10000)
                            .build()
                    )
                    .build()
            )
            .build();
        
        let session = client.checkout().create_session(request).await?;
        println!("Checkout URL: {}", session.url);
    }
    
    Ok(())
}
```

### Managing Orders

```rust
use dintero::{DinteroClient, DinteroConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DinteroClient::from_env()?;
    
    #[cfg(feature = "orders")]
    {
        // Get an order
        let order = client.orders().get_order("order-id").await?;
        println!("Order status: {:?}", order.status);
        
        // List orders
        let orders = client.orders().list_orders().await?;
        println!("Found {} orders", orders.len());
    }
    
    Ok(())
}
```

### Payment Operations

```rust
use dintero::{DinteroClient, DinteroConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DinteroClient::from_env()?;
    
    #[cfg(feature = "payments")]
    {
        use dintero_payments::CaptureRequest;
        
        // Capture a payment
        let capture = CaptureRequest::builder()
            .amount(5000)
            .build();
        
        let result = client.payments()
            .capture_payment("transaction-id", capture)
            .await?;
        
        println!("Captured: {}", result.amount);
    }
    
    Ok(())
}
```

## Examples

The repository includes comprehensive examples in the `dintero/examples/` directory:

- `basic.rs` - Basic client setup and configuration
- `checkout_session.rs` - Creating checkout sessions
- `orders.rs` - Order management operations
- `payments.rs` - Payment and capture operations
- `accounts.rs` - Account management
- `loyalty.rs` - Loyalty program integration
- `insights.rs` - Analytics and reporting

Run an example:

```bash
cargo run --example basic --features checkout
```

## Authentication

The SDK supports multiple authentication methods:

### Environment Variables

```bash
export DINTERO_ACCOUNT_ID="your-account-id"
export DINTERO_CLIENT_ID="your-client-id"
export DINTERO_CLIENT_SECRET="your-client-secret"
```

### Configuration Object

```rust
use dintero::DinteroConfig;

let config = DinteroConfig::new(
    "account-id",
    "client-id",
    "client-secret"
);
```

### Builder Pattern

```rust
use dintero::DinteroConfig;

let config = DinteroConfig::builder()
    .account_id("account-id")
    .client_id("client-id")
    .client_secret("client-secret")
    .base_url("https://api.dintero.com") // optional
    .build();
```

## Error Handling

The SDK uses a custom error type that wraps all possible errors:

```rust
use dintero::{DinteroClient, DinteroError};

async fn example() -> Result<(), DinteroError> {
    let client = DinteroClient::from_env()?;
    
    match client.checkout().get_session("session-id").await {
        Ok(session) => println!("Session found: {}", session.id),
        Err(DinteroError::NotFound) => println!("Session not found"),
        Err(DinteroError::Authentication) => println!("Authentication failed"),
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}
```

## Testing

Run all tests:

```bash
cargo test --all-features
```

Run tests for a specific feature:

```bash
cargo test --features checkout
```

## Documentation

Full API documentation is available at [docs.rs/dintero](https://docs.rs/dintero).

Build documentation locally:

```bash
cargo doc --all-features --open
```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Copyright

Copyright © 2024 Budna Marketplace AB. All rights reserved.

Author: Marcus Cvjeticanin

## Support

- [Dintero Documentation](https://docs.dintero.com)
- [GitHub Issues](https://github.com/budna-platform/dintero-rust-sdk/issues)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.
