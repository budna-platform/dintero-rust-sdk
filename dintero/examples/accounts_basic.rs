use dintero::{Config, DinteroClient, Environment};

#[cfg(feature = "accounts")]
use dintero::accounts::{UpdateAccountRequest, UpdateProfileRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "accounts")]
    {
        tracing_subscriber::fmt::init();

        let config = Config::builder("T12345678")
            .environment(Environment::Test)
            .api_key("your_api_key")
            .timeout_secs(30)
            .build()?;

        let client = DinteroClient::new(config)?;
        let accounts = client.accounts();

        println!("=== Dintero Accounts API Example ===\n");

        println!("1. Getting current session...");
        match accounts.get_session().await {
            Ok(session) => {
                println!("   Session ID: {}", session.id);
                if let Some(account_id) = &session.account_id {
                    println!("   Account ID: {}", account_id);
                }
                if let Some(profile_id) = &session.profile_id {
                    println!("   Profile ID: {}", profile_id);
                }
            }
            Err(e) => println!("   Error: {:?}", e),
        }

        println!("\n2. Listing accounts...");
        match accounts.list_accounts(None).await {
            Ok(account_list) => {
                println!("   Found {} accounts", account_list.accounts.len());
                for account in account_list.accounts.iter().take(3) {
                    println!("   - {} (ID: {})", account.name, account.id);
                    if let Some(email) = &account.email {
                        println!("     Email: {}", email);
                    }
                    if let Some(country) = &account.country {
                        println!("     Country: {}", country);
                    }
                }
                if let Some(next_token) = account_list.next_page_token {
                    println!("   Next page token: {}", next_token);
                }
            }
            Err(e) => println!("   Error: {:?}", e),
        }

        let account_id = "T12345678";

        println!("\n3. Getting specific account...");
        match accounts.get_account(account_id).await {
            Ok(account) => {
                println!("   Account Name: {}", account.name);
                println!("   Account ID: {}", account.id);
                if let Some(org) = &account.organization_number {
                    println!("   Organization Number: {}", org);
                }
                if let Some(currency) = &account.currency {
                    println!("   Currency: {}", currency);
                }
            }
            Err(e) => println!("   Error: {:?}", e),
        }

        println!("\n4. Updating account...");
        let update_request = UpdateAccountRequest::new()
            .name("Updated Account Name")
            .email("updated@example.com")
            .phone("+47 98765432");

        match accounts.update_account(account_id, update_request).await {
            Ok(updated) => {
                println!("   Successfully updated account");
                println!("   New name: {}", updated.name);
                if let Some(email) = updated.email {
                    println!("   New email: {}", email);
                }
            }
            Err(e) => println!("   Error: {:?}", e),
        }

        println!("\n5. Listing profiles for account...");
        match accounts.list_profiles(account_id, None).await {
            Ok(profile_list) => {
                println!("   Found {} profiles", profile_list.profiles.len());
                for profile in profile_list.profiles.iter().take(3) {
                    println!("   - {} (ID: {})", profile.name, profile.id);
                    if let Some(logo) = &profile.logo_url {
                        println!("     Logo URL: {}", logo);
                    }
                }
            }
            Err(e) => println!("   Error: {:?}", e),
        }

        let profile_id = "P12345678";

        println!("\n6. Getting specific profile...");
        match accounts.get_profile(account_id, profile_id).await {
            Ok(profile) => {
                println!("   Profile Name: {}", profile.name);
                println!("   Profile ID: {}", profile.id);
                if let Some(settings) = &profile.settings {
                    if let Some(branding) = &settings.branding {
                        if let Some(colors) = &branding.colors {
                            println!("   Brand colors: {} defined", colors.len());
                        }
                    }
                    if let Some(checkout) = &settings.checkout {
                        if let Some(terms) = &checkout.terms_url {
                            println!("   Terms URL: {}", terms);
                        }
                    }
                }
            }
            Err(e) => println!("   Error: {:?}", e),
        }

        println!("\n7. Updating profile...");
        let profile_update = UpdateProfileRequest::new()
            .name("Updated Profile")
            .logo_url("https://example.com/new-logo.png");

        match accounts.update_profile(account_id, profile_id, profile_update).await {
            Ok(updated) => {
                println!("   Successfully updated profile");
                println!("   New name: {}", updated.name);
                if let Some(logo) = updated.logo_url {
                    println!("   New logo URL: {}", logo);
                }
            }
            Err(e) => println!("   Error: {:?}", e),
        }

        println!("\n=== Example Complete ===");
    }

    #[cfg(not(feature = "accounts"))]
    {
        println!("This example requires the 'accounts' feature to be enabled.");
        println!("Run with: cargo run --example accounts_example --features accounts");
    }

    Ok(())
}
