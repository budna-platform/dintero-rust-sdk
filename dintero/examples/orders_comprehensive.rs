use dintero::orders::*;
use dintero::{Config, DinteroClient, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = Config::builder("T12345678")
        .api_key("test_secret_key_abc123")
        .environment(Environment::Test)
        .build()?;

    let client = DinteroClient::new(config)?;
    let orders_client = client.orders();

    println!("🛒 Dintero Orders API - Comprehensive Example\n");

    // ===== DRAFT ORDERS =====
    println!("📝 1. Creating Draft Order...");
    let draft_item = DraftOrderItem {
        line_id: "item-1".to_string(),
        description: "Premium Subscription".to_string(),
        quantity: 1,
        amount: 99900,
        vat_amount: 19980,
        vat: 2500,
        product_id: Some("PROD-001".to_string()),
    };

    let draft_order = CreateDraftOrderRequest::builder()
        .amount(99900)
        .currency("NOK")
        .merchant_reference("DRAFT-2024-001")
        .add_item(draft_item)
        .build()?;

    let created_draft = orders_client.create_draft_order(draft_order).await?;
    println!("   ✅ Draft order created: {}", created_draft.id);

    println!("\n📋 2. Adding Item to Draft Order...");
    let new_item = AddDraftOrderItemRequest::new("item-2", "Setup Fee", 1, 50000, 10000, 2500)
        .with_product_id("SETUP-001");

    orders_client
        .add_draft_order_item(&created_draft.id, new_item)
        .await?;
    println!("   ✅ Item added to draft order");

    println!("\n✔️  3. Completing Draft Order (converting to real order)...");
    let completed_order = orders_client
        .complete_draft_order(&created_draft.id)
        .await?;
    println!("   ✅ Order created from draft: {}", completed_order.id);

    // ===== ORDER OPERATIONS =====
    println!("\n💳 4. Creating Authorization...");
    let auth_request = CreateAuthorizationRequest::new(149900).with_payment_product("vipps");

    let authorization = orders_client
        .create_authorization(&completed_order.id, auth_request)
        .await?;
    println!("   ✅ Authorization created: {}", authorization.id);

    println!("\n📸 5. Creating Capture...");
    let capture_request = CreateCaptureRequest::new(100000);

    let capture = orders_client
        .create_capture(&completed_order.id, capture_request)
        .await?;
    println!(
        "   ✅ Captured amount: {} {}",
        capture.amount, capture.currency
    );

    println!("\n↩️  6. Creating Refund...");
    let refund_request =
        CreateRefundRequest::new(25000).with_reason("Customer requested partial refund");

    let refund = orders_client
        .create_refund(&completed_order.id, refund_request)
        .await?;
    println!(
        "   ✅ Refunded amount: {} {}",
        refund.amount, refund.currency
    );

    // ===== ORDER SESSIONS =====
    println!("\n🔗 7. Creating Order Session (for checkout)...");
    let session_request =
        CreateOrderSessionRequest::new().with_return_url("https://example.com/order-complete");

    let session = orders_client
        .create_order_session(&completed_order.id, session_request)
        .await?;
    println!("   ✅ Session created: {}", session.id);
    if let Some(url) = &session.url {
        println!("   🌐 Checkout URL: {}", url);
    }

    // ===== COMMENTS & EVENTS =====
    println!("\n💬 8. Adding Comment to Order...");
    let comment_request = CreateCommentRequest::new("Customer requested express shipping");

    let comment = orders_client
        .create_comment(&completed_order.id, comment_request)
        .await?;
    println!("   ✅ Comment added: {}", comment.id);

    println!("\n📅 9. Creating Custom Event...");
    let event_data = serde_json::json!({
        "source": "example_app",
        "action": "order_processed"
    });

    let event_request = CreateEventRequest::new("custom.order.processed").with_data(event_data);

    let event = orders_client
        .create_event(&completed_order.id, event_request)
        .await?;
    println!("   ✅ Event created: {} ({})", event.id, event.event_type);

    // ===== LISTING OPERATIONS =====
    println!("\n📊 10. Listing All Authorizations...");
    let auths = orders_client
        .list_authorizations(&completed_order.id)
        .await?;
    println!(
        "   ✅ Found {} authorization(s)",
        auths.authorizations.len()
    );

    println!("\n📊 11. Listing All Captures...");
    let captures = orders_client.list_captures(&completed_order.id).await?;
    println!("   ✅ Found {} capture(s)", captures.captures.len());

    println!("\n📊 12. Listing All Refunds...");
    let refunds = orders_client.list_refunds(&completed_order.id).await?;
    println!("   ✅ Found {} refund(s)", refunds.refunds.len());

    println!("\n📊 13. Getting Order Events...");
    let events = orders_client.get_events(&completed_order.id).await?;
    println!("   ✅ Found {} event(s)", events.events.len());

    // ===== ORDER STATE MANAGEMENT =====
    println!("\n🔒 14. Closing Order...");
    let closed_order = orders_client.close_order(&completed_order.id).await?;
    println!("   ✅ Order closed: {:?}", closed_order.status);

    println!("\n🔓 15. Reopening Order...");
    let reopened_order = orders_client.open_order(&completed_order.id).await?;
    println!("   ✅ Order reopened: {:?}", reopened_order.status);

    // ===== LISTING ORDERS WITH FILTERS =====
    println!("\n📋 16. Listing Orders with Filters...");
    let list_params = ListOrdersParams::builder()
        .limit(10)
        .status(OrderStatus::Captured)
        .build();

    let orders = orders_client.list_orders(list_params).await?;
    println!("   ✅ Found {} order(s)", orders.orders.len());

    // ===== CUSTOMER VIEW =====
    println!("\n👤 17. Listing Customer Orders...");
    let customer_id = "customer-123";
    let customer_orders = orders_client.list_customer_orders(customer_id).await?;
    println!(
        "   ✅ Customer has {} order(s)",
        customer_orders.orders.len()
    );

    // ===== CANCELLATION =====
    println!("\n❌ 18. Creating Cancellation...");
    let cancel_request = CreateCancellationRequest::new().with_reason("Test cancellation");

    let cancellation = orders_client
        .create_cancellation(&completed_order.id, cancel_request)
        .await?;
    println!("   ✅ Cancellation created: {}", cancellation.id);

    // ===== FINAL STATUS =====
    println!("\n🔍 19. Getting Final Order State...");
    let final_order = orders_client.get_order(&completed_order.id).await?;
    println!("   ✅ Order ID: {}", final_order.id);
    println!("   ✅ Status: {:?}", final_order.status);
    println!(
        "   ✅ Amount: {} {}",
        final_order.amount, final_order.currency
    );

    println!("\n✨ Orders API example complete!");
    println!("\n🎯 Operations demonstrated:");
    println!("   • Draft Orders (create, add items, complete)");
    println!("   • Authorizations (create, get, list)");
    println!("   • Captures (create, get, list)");
    println!("   • Refunds (create, get, list)");
    println!("   • Order Sessions (create, get, list)");
    println!("   • Comments (create)");
    println!("   • Events (create, list)");
    println!("   • Order State (close, open)");
    println!("   • Listing & Filtering");
    println!("   • Customer Views");
    println!("   • Cancellations");

    Ok(())
}
