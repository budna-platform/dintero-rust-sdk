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

    println!("=== Dintero Accounts API Examples ===\n");

    println!("1. Get Current Session");
    println!("   Getting session information...");
    match accounts.get_session().await {
        Ok(session) => {
            println!("   ✓ Session ID: {}", session.id);
            if let Some(account_id) = &session.account_id {
                println!("   ✓ Account ID: {}", account_id);
            }
            if let Some(profile_id) = &session.profile_id {
                println!("   ✓ Profile ID: {}", profile_id);
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    println!("\n2. List Accounts");
    println!("   Fetching all accounts...");
    match accounts.list_accounts(None).await {
        Ok(account_list) => {
            println!("   ✓ Found {} account(s)", account_list.accounts.len());
            for account in &account_list.accounts {
                println!("     - {} ({})", account.name, account.id);
                if let Some(country) = &account.country {
                    println!("       Country: {}", country);
                }
                if let Some(currency) = &account.currency {
                    println!("       Currency: {}", currency);
                }
            }
            if let Some(next_token) = account_list.next_page_token {
                println!(
                    "   ℹ More results available (next_page_token: {})",
                    next_token
                );
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    let account_id = "T12345678";

    println!("\n3. Get Specific Account");
    println!("   Fetching account: {}", account_id);
    match accounts.get_account(account_id).await {
        Ok(account) => {
            println!("   ✓ Name: {}", account.name);
            println!("   ✓ ID: {}", account.id);
            if let Some(org_number) = &account.organization_number {
                println!("   ✓ Organization Number: {}", org_number);
            }
            if let Some(email) = &account.email {
                println!("   ✓ Email: {}", email);
            }
            if let Some(phone) = &account.phone {
                println!("   ✓ Phone: {}", phone);
            }
            if let Some(settings) = &account.settings {
                if let Some(payment_methods) = &settings.payment_methods {
                    println!("   ✓ Payment Methods: {}", payment_methods.join(", "));
                }
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    println!("\n4. Update Account");
    println!("   Updating account information...");

    use dintero::accounts::UpdateAccountRequest;
    let update_request = UpdateAccountRequest::new()
        .name("Updated Account Name")
        .email("updated@example.com")
        .phone("+4712345678");

    match accounts.update_account(account_id, update_request).await {
        Ok(account) => {
            println!("   ✓ Account updated successfully");
            println!("   ✓ Name: {}", account.name);
            if let Some(email) = &account.email {
                println!("   ✓ Email: {}", email);
            }
            if let Some(updated_at) = &account.updated_at {
                println!("   ✓ Updated at: {}", updated_at);
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    println!("\n5. List Profiles for Account");
    println!("   Fetching profiles for account: {}", account_id);
    match accounts.list_profiles(account_id, None).await {
        Ok(profile_list) => {
            println!("   ✓ Found {} profile(s)", profile_list.profiles.len());
            for profile in &profile_list.profiles {
                println!("     - {} ({})", profile.name, profile.id);
                if let Some(logo_url) = &profile.logo_url {
                    println!("       Logo: {}", logo_url);
                }
                if let Some(settings) = &profile.settings {
                    if let Some(branding) = &settings.branding {
                        if let Some(colors) = &branding.colors {
                            println!("       Branding colors: {:?}", colors);
                        }
                    }
                }
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    let profile_id = "profile_123";

    println!("\n6. Get Specific Profile");
    println!("   Fetching profile: {}", profile_id);
    match accounts.get_profile(account_id, profile_id).await {
        Ok(profile) => {
            println!("   ✓ Name: {}", profile.name);
            println!("   ✓ ID: {}", profile.id);
            if let Some(account_id) = &profile.account_id {
                println!("   ✓ Account ID: {}", account_id);
            }
            if let Some(settings) = &profile.settings {
                if let Some(checkout) = &settings.checkout {
                    if let Some(terms_url) = &checkout.terms_url {
                        println!("   ✓ Terms URL: {}", terms_url);
                    }
                    if let Some(privacy_url) = &checkout.privacy_url {
                        println!("   ✓ Privacy URL: {}", privacy_url);
                    }
                }
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    println!("\n7. Update Profile");
    println!("   Updating profile...");

    use dintero::accounts::{CheckoutSettings, ProfileSettings, UpdateProfileRequest};

    let checkout_settings = CheckoutSettings {
        terms_url: Some("https://example.com/terms".to_string()),
        privacy_url: Some("https://example.com/privacy".to_string()),
    };

    let profile_settings = ProfileSettings {
        branding: None,
        checkout: Some(checkout_settings),
    };

    let update_request = UpdateProfileRequest::new()
        .name("Updated Profile")
        .logo_url("https://example.com/logo.png")
        .settings(profile_settings);

    match accounts.update_profile(account_id, profile_id, update_request).await {
        Ok(profile) => {
            println!("   ✓ Profile updated successfully");
            println!("   ✓ Name: {}", profile.name);
            if let Some(logo_url) = &profile.logo_url {
                println!("   ✓ Logo: {}", logo_url);
            }
        }
        Err(e) => println!("   ✗ Error: {}", e),
    }

    println!("\n=== All Examples Complete ===");

    Ok(())
}
