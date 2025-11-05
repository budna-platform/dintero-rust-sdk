use dintero::{Config, DinteroClient, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = Config::builder("T12345678")
        .environment(Environment::Test)
        .api_key("your_api_key_here")
        .timeout_secs(30)
        .build()?;

    let client = DinteroClient::new(config)?;
    let accounts = client.accounts();

    println!("=== Dintero Account Management Examples ===\n");

    println!("--- Account Operations ---");
    match accounts.get_account_details().await {
        Ok(account) => {
            println!("✓ Account: {}", account.name);
            println!("  ID: {}", account.id);
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\n--- Location Management ---");
    match accounts.list_locations().await {
        Ok(locations) => {
            println!("✓ Found {} location(s)", locations.len());
            for location in locations {
                println!("  - {} (ID: {})", location.name, location.id);
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\n--- User Management ---");
    match accounts.list_users().await {
        Ok(users) => {
            println!("✓ Found {} user(s)", users.len());
            for user in users {
                println!("  - {} ({}) - Roles: {}", user.name.unwrap_or_default(), user.email, user.roles.join(", "));
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\n--- OAuth Clients ---");
    match accounts.list_oauth_clients().await {
        Ok(clients) => {
            println!("✓ Found {} OAuth client(s)", clients.len());
            for client in clients {
                println!("  - {} ({})", client.client_name, client.client_id);
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\n--- Gateway Connections ---");
    match accounts.list_gateway_connections().await {
        Ok(gateways) => {
            println!("✓ Found {} gateway connection(s)", gateways.len());
            for gateway in gateways {
                println!("  - {} ({}) - Enabled: {}", gateway.name, gateway.gateway_type, gateway.enabled);
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    println!("\n=== Management Examples Complete ===");
    Ok(())
}
