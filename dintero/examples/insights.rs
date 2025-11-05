use chrono::{Duration, Utc};
use dintero::{Config, DinteroClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config =
        Config::builder("T12345678").api_key("test_secret_key_1234567890abcdef").build()?;

    let client = DinteroClient::new(config)?;
    let insights = client.insights();

    let now = Utc::now();
    let thirty_days_ago = now - Duration::days(30);

    println!("=== Dintero Insights API Example ===\n");

    println!("--- KPIs ---\n");

    let kpi_params = dintero_insights::KpiQueryParams::new(thirty_days_ago, now)
        .with_group_by("day".to_string());

    println!("1. Getting checkout transaction status KPIs...");
    match insights.kpis().get_checkout_transaction_status(kpi_params.clone()).await {
        Ok(response) => {
            println!(
                "Period: {} to {}",
                response.period_start, response.period_end
            );
            println!("Transaction statuses:");
            for status_kpi in response.data {
                println!(
                    "  - {}: {} transactions, {} {} total",
                    status_kpi.status, status_kpi.count, status_kpi.amount, status_kpi.currency
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("2. Getting transaction KPIs...");
    match insights.kpis().get_transactions(kpi_params.clone()).await {
        Ok(response) => {
            println!(
                "Period: {} to {}",
                response.period_start, response.period_end
            );
            let kpi = response.data;
            println!("Total transactions: {}", kpi.total_transactions);
            println!("Total amount: {} {}", kpi.total_amount, kpi.currency);
            println!(
                "Successful: {} transactions ({} {})",
                kpi.successful_transactions, kpi.successful_amount, kpi.currency
            );
            println!("Failed: {} transactions", kpi.failed_transactions);
            println!(
                "Refunded: {} transactions ({} {})",
                kpi.refunded_transactions, kpi.refunded_amount, kpi.currency
            );
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("3. Getting payment method KPIs...");
    match insights.kpis().get_payment_methods(kpi_params.clone()).await {
        Ok(response) => {
            println!(
                "Period: {} to {}",
                response.period_start, response.period_end
            );
            println!("Payment methods:");
            for method in response.data {
                println!(
                    "  - {}: {} transactions ({:.1}%), {} {} total",
                    method.payment_method,
                    method.count,
                    method.percentage,
                    method.amount,
                    method.currency
                );
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("4. Getting revenue KPIs...");
    match insights.kpis().get_revenue(kpi_params).await {
        Ok(response) => {
            println!(
                "Period: {} to {}",
                response.period_start, response.period_end
            );
            println!(
                "Total revenue: {} {}",
                response.total_revenue, response.currency
            );
            println!("Daily breakdown:");
            for revenue in response.data.iter().take(7) {
                println!(
                    "  - {}: {} {} ({} transactions)",
                    revenue.date, revenue.revenue, revenue.currency, revenue.transaction_count
                );
            }
            if response.data.len() > 7 {
                println!("  ... and {} more days", response.data.len() - 7);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("\n--- Report Configurations ---\n");

    println!("5. Listing report configurations...");
    match insights.reports().list_configurations().await {
        Ok(configs) => {
            println!("Found {} report configurations", configs.len());
            for config in configs.iter().take(3) {
                println!(
                    "  - {} ({}): {}",
                    config.name,
                    config.report_type,
                    config.description.as_ref().unwrap_or(&"No description".to_string())
                );
                if let Some(schedule) = &config.schedule {
                    println!(
                        "    Schedule: {:?} at {}",
                        schedule.frequency, schedule.time
                    );
                }
                println!("    Recipients: {}", config.recipients.join(", "));
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();

    println!("6. Creating a new report configuration...");
    let create_request = dintero_insights::CreateReportConfigurationRequest {
        report_type: "transaction_summary".to_string(),
        name: "Weekly Transaction Report".to_string(),
        description: Some("Automated weekly transaction summary report".to_string()),
        schedule: Some(dintero_insights::ReportSchedule {
            frequency: dintero_insights::ScheduleFrequency::Weekly,
            time: "09:00".to_string(),
            day_of_week: Some(1),
            day_of_month: None,
        }),
        recipients: vec!["finance@example.com".to_string()],
        parameters: std::collections::HashMap::new(),
    };

    match insights.reports().create_configuration(create_request).await {
        Ok(config) => {
            println!("Created report configuration: {}", config.id);
            println!("  Name: {}", config.name);
            println!("  Type: {}", config.report_type);

            println!("\n7. Updating the report configuration...");
            let update_request = dintero_insights::UpdateReportConfigurationRequest {
                name: Some("Updated Weekly Transaction Report".to_string()),
                description: Some("Modified description".to_string()),
                schedule: None,
                recipients: Some(vec![
                    "finance@example.com".to_string(),
                    "accounting@example.com".to_string(),
                ]),
                parameters: None,
            };

            match insights.reports().update_configuration(&config.id, update_request).await {
                Ok(updated) => {
                    println!("Updated report configuration: {}", updated.id);
                    println!("  Name: {}", updated.name);
                    println!("  Recipients: {}", updated.recipients.join(", "));
                }
                Err(e) => println!("Error updating: {}", e),
            }

            println!("\n8. Getting the report configuration...");
            match insights.reports().get_configuration(&config.id).await {
                Ok(fetched) => {
                    println!("Fetched report configuration: {}", fetched.id);
                    println!("  Name: {}", fetched.name);
                    println!("  Created: {}", fetched.created_at);
                    println!("  Updated: {}", fetched.updated_at);
                }
                Err(e) => println!("Error fetching: {}", e),
            }

            println!("\n9. Deleting the report configuration...");
            match insights.reports().delete_configuration(&config.id).await {
                Ok(_) => println!("Successfully deleted report configuration"),
                Err(e) => println!("Error deleting: {}", e),
            }
        }
        Err(e) => println!("Error creating report: {}", e),
    }

    println!("\n=== Example Complete ===");

    Ok(())
}
