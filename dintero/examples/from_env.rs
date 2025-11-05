use dintero::{DinteroClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let client = DinteroClient::from_env()?;

    println!("Dintero client initialized from environment variables!");
    println!("Account ID: {}", client.http().account_id());

    Ok(())
}
