use dintero::checkout::transactions::{CaptureRequest, RefundRequest, VoidRequest};
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

    let transaction_id = "tx_123456789";

    println!("Fetching transaction {}...", transaction_id);
    match checkout.get_transaction(transaction_id).await {
        Ok(transaction) => {
            println!("✅ Transaction found!");
            println!("ID: {}", transaction.id);
            println!("Status: {:?}", transaction.status);
            println!("Amount: {} {}", transaction.amount, transaction.currency);
        }
        Err(e) => {
            println!("❌ Failed to get transaction: {}", e);
        }
    }

    println!("\nCapturing transaction...");
    let capture = CaptureRequest::new(10000);
    match checkout.capture_transaction(transaction_id, capture).await {
        Ok(transaction) => {
            println!("✅ Transaction captured!");
            println!("Status: {:?}", transaction.status);
        }
        Err(e) => {
            println!("❌ Failed to capture: {}", e);
        }
    }

    println!("\nRefunding transaction...");
    let refund = RefundRequest::new(5000).with_reason("Customer request");

    match checkout.refund_transaction(transaction_id, refund).await {
        Ok(transaction) => {
            println!("✅ Transaction refunded!");
            println!("Status: {:?}", transaction.status);
        }
        Err(e) => {
            println!("❌ Failed to refund: {}", e);
        }
    }

    println!("\nVoiding transaction...");
    let void_req = VoidRequest::new().with_reason("Cancelled by merchant");

    match checkout.void_transaction(transaction_id, void_req).await {
        Ok(transaction) => {
            println!("✅ Transaction voided!");
            println!("Status: {:?}", transaction.status);
        }
        Err(e) => {
            println!("❌ Failed to void: {}", e);
        }
    }

    Ok(())
}
