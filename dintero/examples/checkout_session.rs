use dintero::checkout::sessions::{CreateSessionRequest, Order, OrderItem};
use dintero::{Config, DinteroClient, Environment, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::builder("T12345678")
        .api_key("your_api_key_here")
        .environment(Environment::Test)
        .build()?;

    let _client = DinteroClient::new(config)?;

    let item = OrderItem::new("item-1", "line-1", "Product 1", 2, 20000, 4000, 25);

    let order = Order::builder()
        .amount(20000)
        .currency("NOK")
        .merchant_reference("order-123")
        .add_item(item)
        .vat_amount(4000)
        .build();

    let session_request = CreateSessionRequest::builder()
        .order(order)
        .return_url("https://example.com/return")
        .callback_url("https://example.com/callback")
        .build()
        .expect("Failed to build session request");

    println!(
        "Created session request with order amount: {}",
        session_request.order.amount
    );
    println!("Currency: {}", session_request.order.currency);

    Ok(())
}
