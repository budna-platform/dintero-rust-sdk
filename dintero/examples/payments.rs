use dintero::payments::*;
use dintero::{Config, DinteroClient, Environment};

type StdResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> StdResult<()> {
    tracing_subscriber::fmt::init();

    let config = Config::builder("T12345678")
        .api_key("test_secret_key_abc123")
        .environment(Environment::Test)
        .build()?;

    let client = DinteroClient::new(config)?;
    let payments_client = client.payments();

    println!("💰 Dintero Payments API - Comprehensive Example\n");

    // ===== TRANSACTIONS =====
    println!("📋 1. Listing Transactions...");
    let list_params =
        ListTransactionsParams::builder().limit(10).status(TransactionStatus::Authorized).build();

    let transactions = payments_client.list_transactions(list_params).await?;
    println!(
        "   ✅ Found {} transaction(s)",
        transactions.transactions.len()
    );

    if let Some(transaction) = transactions.transactions.first() {
        let transaction_id = &transaction.id;

        println!("\n🔍 2. Getting Transaction Details...");
        let txn = payments_client.get_transaction(transaction_id).await?;
        println!("   ✅ Transaction ID: {}", txn.id);
        println!("   ✅ Amount: {} {}", txn.amount, txn.currency);
        println!("   ✅ Status: {:?}", txn.status);

        println!("\n✏️  3. Updating Transaction Metadata...");
        let update_request = UpdateTransactionRequest::new()
            .with_merchant_reference("TXN-2024-001")
            .with_metadata(serde_json::json!({
                "customer_id": "CUST-12345",
                "order_number": "ORD-98765"
            }));

        let updated_txn =
            payments_client.update_transaction(transaction_id, update_request).await?;
        println!("   ✅ Transaction updated");
        if let Some(ref_id) = &updated_txn.merchant_reference {
            println!("   ✅ Merchant reference: {}", ref_id);
        }

        println!("\n📸 4. Capturing Transaction...");
        let capture_request = CaptureTransactionRequest::new(txn.amount);

        let captured = payments_client.capture_transaction(transaction_id, capture_request).await?;
        println!("   ✅ Captured: {} {}", captured.amount, captured.currency);

        println!("\n↩️  5. Refunding Transaction (partial)...");
        let refund_amount = txn.amount / 2;
        let refund_request = RefundTransactionRequest::new(refund_amount)
            .with_reason("Customer requested partial refund");

        let refunded = payments_client.refund_transaction(transaction_id, refund_request).await?;
        println!("   ✅ Refunded: {} {}", refunded.amount, refunded.currency);
    }

    // Demo void on a different transaction
    println!("\n❌ 6. Voiding Authorization...");
    if let Some(auth_txn) =
        transactions.transactions.iter().find(|t| t.status == TransactionStatus::Authorized)
    {
        let void_request = VoidTransactionRequest::new().with_reason("Customer cancelled order");

        let voided = payments_client.void_transaction(&auth_txn.id, void_request).await?;
        println!("   ✅ Transaction voided: {}", voided.id);
    } else {
        println!("   ⚠️  No authorized transactions to void");
    }

    println!("\n⏰ 7. Extending Authorization...");
    if let Some(auth_txn) =
        transactions.transactions.iter().find(|t| t.status == TransactionStatus::Authorized)
    {
        let extend_request = ExtendAuthorizationRequest::new(7);

        let extended = payments_client.extend_authorization(&auth_txn.id, extend_request).await?;
        println!(
            "   ✅ Authorization extended for transaction: {}",
            extended.id
        );
    } else {
        println!("   ⚠️  No authorized transactions to extend");
    }

    // ===== SETTLEMENTS =====
    println!("\n💼 8. Listing Settlements...");
    let settlements = payments_client.list_settlements().await?;
    println!(
        "   ✅ Found {} settlement(s)",
        settlements.settlements.len()
    );

    for settlement in settlements.settlements.iter().take(3) {
        println!(
            "   📊 Settlement {}: {} {}",
            settlement.id, settlement.amount, settlement.currency
        );
    }

    println!("\n⚙️  9. Creating Settlement Report Configuration...");
    let report_config = CreateSettlementReportConfigRequest::new()
        .with_email("finance@example.com")
        .with_file_format("CSV")
        .enabled(true);

    let config_created = payments_client.create_settlement_report_config(report_config).await?;
    println!(
        "   ✅ Settlement report config created: {}",
        config_created.id
    );

    println!("\n📋 10. Listing Settlement Report Configs...");
    let configs = payments_client.list_settlement_report_configs().await?;
    println!("   ✅ Found {} configuration(s)", configs.len());

    if let Some(config) = configs.first() {
        println!("\n🔍 11. Getting Settlement Report Config Details...");
        let config_details = payments_client.get_settlement_report_config(&config.id).await?;
        println!("   ✅ Config ID: {}", config_details.id);
        println!("   ✅ Enabled: {}", config_details.enabled);

        println!("\n✏️  12. Updating Settlement Report Config...");
        let update_config = UpdateSettlementReportConfigRequest::new()
            .with_email("accounting@example.com")
            .enabled(true);

        let updated_config =
            payments_client.update_settlement_report_config(&config.id, update_config).await?;
        println!("   ✅ Config updated");
        if let Some(email) = &updated_config.email {
            println!("   ✅ New email: {}", email);
        }
    }

    // ===== PAYOUT DESTINATIONS =====
    println!("\n🏦 13. Creating Payout Destination...");
    let payout_dest =
        CreatePayoutDestinationRequest::new("Main Business Account", "NO9386011117947")
            .with_bank_code("DNBANOKK");

    let created_dest = payments_client.create_payout_destination(payout_dest).await?;
    println!("   ✅ Payout destination created: {}", created_dest.id);

    println!("\n📋 14. Listing Payout Destinations...");
    let destinations = payments_client.list_payout_destinations().await?;
    println!(
        "   ✅ Found {} payout destination(s)",
        destinations.payout_destinations.len()
    );

    if let Some(dest) = destinations.payout_destinations.first() {
        println!("\n💵 15. Getting Payout Balance...");
        let balance = payments_client.get_payout_balance(&dest.id).await?;
        println!("   ✅ Currency: {}", balance.currency);
        println!("   ✅ Available: {}", balance.available);
        println!("   ✅ Pending: {}", balance.pending);

        println!("\n📊 16. Listing Payout Transfers...");
        let transfers = payments_client.list_payout_transfers(&dest.id).await?;
        println!("   ✅ Found {} transfer(s)", transfers.transfers.len());

        println!("\n💸 17. Creating Payout Transfer...");
        let transfer_request = CreatePayoutTransferRequest::new(100000, "NOK", dest.id.clone());

        let created_transfer = payments_client.create_payout_transfer(transfer_request).await?;
        println!("   ✅ Transfer created: {}", created_transfer.id);
        println!(
            "   ✅ Amount: {} {}",
            created_transfer.amount, created_transfer.currency
        );
    }

    // ===== CLEANUP =====
    if let Some(config) = configs.first() {
        println!("\n🗑️  18. Deleting Settlement Report Config...");
        payments_client.delete_settlement_report_config(&config.id).await?;
        println!("   ✅ Config deleted");
    }

    println!("\n✨ Payments API example complete!");
    println!("\n🎯 Operations demonstrated:");
    println!("   • Transactions (list, get, update, capture, refund, void, extend)");
    println!("   • Settlements (list)");
    println!("   • Settlement Reports (create, get, list, update, delete)");
    println!("   • Payout Destinations (create, list)");
    println!("   • Payout Balances (get)");
    println!("   • Payout Transfers (create, list)");

    Ok(())
}
