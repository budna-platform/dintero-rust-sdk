use dintero::checkout::sessions::{CreateSessionRequest, Order, OrderItem};
use dintero::checkout::CheckoutOperations;
use dintero::{Config, DinteroClient, Environment, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::builder("T12345678")
        .api_key("your_api_key_here")
        .environment(Environment::Test)
        .build()?;

    let client = DinteroClient::new(config)?;
    let checkout = client.checkout();

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

    println!("Creating checkout session...");

    match checkout.create_session(session_request).await {
        Ok(session) => {
            println!("✅ Session created successfully!");
            println!("Session ID: {}", session.id);
            println!("Session URL: {}", session.url);

            if let Some(status) = session.status {
                println!("Status: {:?}", status);
            }
        }
        Err(e) => {
            println!("❌ Failed to create session: {}", e);
        }
    }

    Ok(())
}
