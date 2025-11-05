use dintero::{Config, DinteroClient, Environment, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::builder("T12345678")
        .api_key("your_api_key_here")
        .environment(Environment::Test)
        .timeout_secs(30)
        .build()?;

    let client = DinteroClient::new(config)?;

    println!("Dintero client initialized successfully!");
    println!("Account ID: {}", client.http().account_id());

    Ok(())
}
