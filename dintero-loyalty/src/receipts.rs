use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::error::Result;
use crate::client::LoyaltyClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub id: Uuid,
    pub customer_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub transaction_id: String,
    pub amount: i64,
    pub currency: String,
    pub items: Vec<ReceiptItem>,
    pub payment_method: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptItem {
    pub id: String,
    pub product_id: Option<Uuid>,
    pub name: String,
    pub quantity: i32,
    pub unit_price: i64,
    pub total_amount: i64,
    pub tax_rate: Option<f64>,
    pub discount_amount: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReceiptRequest {
    pub customer_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub transaction_id: String,
    pub amount: i64,
    pub currency: String,
    pub items: Vec<CreateReceiptItemRequest>,
    pub payment_method: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReceiptItemRequest {
    pub product_id: Option<Uuid>,
    pub name: String,
    pub quantity: i32,
    pub unit_price: i64,
    pub total_amount: i64,
    pub tax_rate: Option<f64>,
    pub discount_amount: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct ListReceiptsRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub customer_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
}

impl LoyaltyClient {
    pub async fn create_receipt(&self, req: CreateReceiptRequest) -> Result<Receipt> {
        let url = self.url("/receipts");
        let response = self.http()
            .post(&url)
            .json(&req)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn get_receipt(&self, receipt_id: &Uuid) -> Result<Receipt> {
        let url = self.url(&format!("/receipts/{}", receipt_id));
        let response = self.http()
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn list_receipts(&self, req: ListReceiptsRequest) -> Result<Vec<Receipt>> {
        let mut url = self.url("/receipts");
        let mut params = vec![];
        
        if let Some(limit) = req.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = req.offset {
            params.push(format!("offset={}", offset));
        }
        if let Some(customer_id) = req.customer_id {
            params.push(format!("customer_id={}", customer_id));
        }
        if let Some(location_id) = req.location_id {
            params.push(format!("location_id={}", location_id));
        }
        
        if !params.is_empty() {
            url.push_str("?");
            url.push_str(&params.join("&"));
        }
        
        let response = self.http()
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }
}
