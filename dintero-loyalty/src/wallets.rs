use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::types::PaginatedResponse;
use crate::error::Result;
use crate::client::LoyaltyClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualCard {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub card_type: CardType,
    pub balance: i64,
    pub currency: String,
    pub status: CardStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardType {
    GiftCard,
    LoyaltyCard,
    PrepaidCard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardStatus {
    Active,
    Inactive,
    Expired,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualCardRequest {
    pub customer_id: Uuid,
    pub card_type: CardType,
    pub balance: i64,
    pub currency: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVirtualCardRequest {
    pub status: Option<CardStatus>,
    pub expires_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardTransaction {
    pub id: Uuid,
    pub card_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub transaction_type: TransactionType,
    pub amount: i64,
    pub balance_after: i64,
    pub description: Option<String>,
    pub reference_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Credit,
    Debit,
    Refund,
    Adjustment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardTransactionRequest {
    pub card_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: i64,
    pub description: Option<String>,
    pub reference_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
pub struct ListCardsRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub customer_id: Option<Uuid>,
    pub status: Option<CardStatus>,
}

#[derive(Debug, Clone, Default)]
pub struct ListTransactionsRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub card_id: Option<Uuid>,
    pub transaction_type: Option<TransactionType>,
}

impl LoyaltyClient {
    pub async fn create_virtual_card(&self, req: CreateVirtualCardRequest) -> Result<VirtualCard> {
        let url = self.url("/wallets/cards");
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

    pub async fn get_virtual_card(&self, card_id: &Uuid) -> Result<VirtualCard> {
        let url = self.url(&format!("/wallets/cards/{}", card_id));
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

    pub async fn update_virtual_card(&self, card_id: &Uuid, req: UpdateVirtualCardRequest) -> Result<VirtualCard> {
        let url = self.url(&format!("/wallets/cards/{}", card_id));
        let response = self.http()
            .put(&url)
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

    pub async fn delete_virtual_card(&self, card_id: &Uuid) -> Result<()> {
        let url = self.url(&format!("/wallets/cards/{}", card_id));
        let response = self.http()
            .delete(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn list_virtual_cards(&self, req: ListCardsRequest) -> Result<PaginatedResponse<VirtualCard>> {
        let mut url = self.url("/wallets/cards");
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

    pub async fn create_card_transaction(&self, req: CreateCardTransactionRequest) -> Result<CardTransaction> {
        let url = self.url("/wallets/transactions");
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

    pub async fn get_card_transaction(&self, transaction_id: &Uuid) -> Result<CardTransaction> {
        let url = self.url(&format!("/wallets/transactions/{}", transaction_id));
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

    pub async fn list_card_transactions(&self, req: ListTransactionsRequest) -> Result<PaginatedResponse<CardTransaction>> {
        let mut url = self.url("/wallets/transactions");
        let mut params = vec![];
        
        if let Some(limit) = req.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = req.offset {
            params.push(format!("offset={}", offset));
        }
        if let Some(card_id) = req.card_id {
            params.push(format!("card_id={}", card_id));
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
