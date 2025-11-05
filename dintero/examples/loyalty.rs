use dintero::loyalty::*;
use dintero::{Config, DinteroClient, Environment};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = Config::builder("T12345678")
        .environment(Environment::Test)
        .api_key(std::env::var("DINTERO_API_KEY").unwrap_or_else(|_| "test_key".to_string()))
        .build()?;

    let client = DinteroClient::new(config)?;
    let loyalty = client.loyalty();

    println!("=== Dintero Loyalty SDK Example ===\n");

    println!("1. Creating a customer...");
    let create_customer_req = CreateCustomerRequest {
        customer_type: CustomerType::Person,
        phone_number: Some(PhoneNumber {
            country_code: "47".to_string(),
            number: "12345678".to_string(),
        }),
        email: Some("customer@example.com".to_string()),
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        company_name: None,
        organization_number: None,
        addresses: None,
        metadata: None,
    };

    match loyalty.customers().create_customer(create_customer_req).await {
        Ok(customer) => {
            println!("✓ Customer created: {} ({})", customer.id, customer.email.as_deref().unwrap_or("N/A"));

            println!("\n2. Creating a virtual card for the customer...");
            let create_card_req = CreateVirtualCardRequest {
                customer_id: customer.id,
                card_type: CardType::LoyaltyCard,
                balance: 10000,
                currency: "NOK".to_string(),
                expires_at: None,
                metadata: None,
            };

            match loyalty.wallets().create_virtual_card(create_card_req).await {
                Ok(card) => {
                    println!("✓ Virtual card created: {} (Balance: {} {})", 
                        card.id, card.balance, card.currency);

                    println!("\n3. Creating a product catalog...");
                    let catalog_req = CreateProductCatalogRequest {
                        name: "Main Catalog".to_string(),
                        description: Some("Our main product catalog".to_string()),
                        active: Some(true),
                    };

                    match loyalty.products().create_product_catalog(catalog_req).await {
                        Ok(catalog) => {
                            println!("✓ Product catalog created: {}", catalog.name);

                            println!("\n4. Adding a product to the catalog...");
                            let product_req = CreateProductItemRequest {
                                catalog_id: catalog.id,
                                sku: "PROD-001".to_string(),
                                name: "Premium Product".to_string(),
                                description: Some("A premium product item".to_string()),
                                price: 5000,
                                currency: "NOK".to_string(),
                                tax_rate: Some(0.25),
                                stock: Some(100),
                                active: Some(true),
                                metadata: None,
                            };

                            match loyalty.products().create_product_item(product_req).await {
                                Ok(product) => {
                                    println!("✓ Product added: {} (SKU: {})", product.name, product.sku);
                                }
                                Err(e) => println!("✗ Failed to create product: {}", e),
                            }
                        }
                        Err(e) => println!("✗ Failed to create catalog: {}", e),
                    }

                    println!("\n5. Creating a discount campaign...");
                    let campaign_req = CreateDiscountCampaignRequest {
                        name: "Summer Sale".to_string(),
                        description: Some("20% off all items".to_string()),
                        code: Some("SUMMER20".to_string()),
                        starts_at: None,
                        ends_at: None,
                        rule_id: None,
                        usage_limit: Some(100),
                        active: Some(true),
                    };

                    match loyalty.discounts().create_discount_campaign(campaign_req).await {
                        Ok(campaign) => {
                            println!("✓ Discount campaign created: {} (Code: {})", 
                                campaign.name, campaign.code.as_deref().unwrap_or("N/A"));
                        }
                        Err(e) => println!("✗ Failed to create campaign: {}", e),
                    }

                    println!("\n6. Creating a receipt...");
                    let receipt_req = CreateReceiptRequest {
                        customer_id: Some(customer.id),
                        location_id: None,
                        transaction_id: "TXN-12345".to_string(),
                        amount: 5000,
                        currency: "NOK".to_string(),
                        items: vec![
                            CreateReceiptItemRequest {
                                product_id: None,
                                name: "Premium Product".to_string(),
                                quantity: 1,
                                unit_price: 5000,
                                total_amount: 5000,
                                tax_rate: Some(0.25),
                                discount_amount: None,
                            },
                        ],
                        payment_method: Some("card".to_string()),
                        metadata: None,
                    };

                    match loyalty.receipts().create_receipt(receipt_req).await {
                        Ok(receipt) => {
                            println!("✓ Receipt created: {} (Amount: {} {})", 
                                receipt.transaction_id, receipt.amount, receipt.currency);
                        }
                        Err(e) => println!("✗ Failed to create receipt: {}", e),
                    }

                    println!("\n7. Creating a location...");
                    let location_req = CreateLocationRequest {
                        name: "Main Store".to_string(),
                        description: Some("Our flagship store".to_string()),
                        address: Some(Address {
                            address_line: Some("123 Main Street".to_string()),
                            address_line_2: None,
                            co_address: None,
                            business_name: None,
                            postal_code: Some("0123".to_string()),
                            postal_place: Some("Oslo".to_string()),
                            country: Some("NO".to_string()),
                            latitude: Some(59.9139),
                            longitude: Some(10.7522),
                        }),
                        contact_email: Some("store@example.com".to_string()),
                        contact_phone: Some("+4712345678".to_string()),
                        opening_hours: None,
                        active: Some(true),
                        metadata: None,
                    };

                    match loyalty.locations().create_location(location_req).await {
                        Ok(location) => {
                            println!("✓ Location created: {}", location.name);
                        }
                        Err(e) => println!("✗ Failed to create location: {}", e),
                    }

                    println!("\n8. Creating a webhook subscription...");
                    let webhook_req = CreateWebhookSubscriptionRequest {
                        url: "https://example.com/webhooks/dintero".to_string(),
                        events: vec![
                            WebhookEvent::CustomerCreated,
                            WebhookEvent::ReceiptCreated,
                            WebhookEvent::TransactionCreated,
                        ],
                        description: Some("Main webhook endpoint".to_string()),
                    };

                    match loyalty.webhooks().create_webhook_subscription(webhook_req).await {
                        Ok(subscription) => {
                            println!("✓ Webhook subscription created: {} events: {}", 
                                subscription.url, subscription.events.len());
                        }
                        Err(e) => println!("✗ Failed to create webhook: {}", e),
                    }

                    println!("\n9. Creating an automation rule...");
                    let automation_req = CreateAutomationRuleRequest {
                        name: "Welcome Gift".to_string(),
                        description: Some("Send welcome gift to new customers".to_string()),
                        requirement: AutomationRequirement {
                            automation_from: None,
                            automation_to: None,
                            events: vec!["customer_add".to_string()],
                            filter: None,
                        },
                        limitation: Some(AutomationLimitation {
                            automation_repeat: Some(1),
                        }),
                        actions: vec![
                            AutomationAction {
                                action_type: "send_email".to_string(),
                                params: None,
                            },
                        ],
                    };

                    match loyalty.automations().create_automation_rule(automation_req).await {
                        Ok(rule) => {
                            println!("✓ Automation rule created: {}", rule.name);
                        }
                        Err(e) => println!("✗ Failed to create automation: {}", e),
                    }
                }
                Err(e) => println!("✗ Failed to create virtual card: {}", e),
            }
        }
        Err(e) => println!("✗ Failed to create customer: {}", e),
    }

    println!("\n=== Example completed ===");
    println!("\nNote: This example demonstrates the full Loyalty API functionality.");
    println!("In a real environment, replace the test credentials with actual ones.");

    Ok(())
}
