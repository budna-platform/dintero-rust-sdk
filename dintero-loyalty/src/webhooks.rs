//! Webhook management for loyalty events.

use crate::client::LoyaltyClient;
use crate::error::Result;
use crate::types::PaginatedResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSubscription {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub url: String,
    pub events: Vec<WebhookEvent>,
    pub secret: String,
    pub active: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEvent {
    CustomerCreated,
    CustomerUpdated,
    CustomerDeleted,
    ReceiptCreated,
    TransactionCreated,
    CardCreated,
    CardUpdated,
    DiscountAssigned,
    CampaignCreated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookSubscriptionRequest {
    pub url: String,
    pub events: Vec<WebhookEvent>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWebhookSubscriptionRequest {
    pub url: Option<String>,
    pub events: Option<Vec<WebhookEvent>>,
    pub active: Option<bool>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDelivery {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub event: WebhookEvent,
    pub payload: serde_json::Value,
    pub status: DeliveryStatus,
    pub attempts: i32,
    pub last_attempt_at: Option<DateTime<Utc>>,
    pub next_attempt_at: Option<DateTime<Utc>>,
    pub response_code: Option<i32>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeliveryStatus {
    Pending,
    Success,
    Failed,
    Retrying,
}

#[derive(Debug, Clone, Default)]
pub struct ListWebhooksRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Default)]
pub struct ListDeliveriesRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub subscription_id: Option<Uuid>,
    pub status: Option<DeliveryStatus>,
}

impl LoyaltyClient {
    pub async fn create_webhook_subscription(
        &self,
        req: CreateWebhookSubscriptionRequest,
    ) -> Result<WebhookSubscription> {
        let url = self.url("/webhooks/subscriptions");
        let response = self.http().post(&url).json(&req).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn get_webhook_subscription(
        &self,
        subscription_id: &Uuid,
    ) -> Result<WebhookSubscription> {
        let url = self.url(&format!("/webhooks/subscriptions/{}", subscription_id));
        let response = self.http().get(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn update_webhook_subscription(
        &self,
        subscription_id: &Uuid,
        req: UpdateWebhookSubscriptionRequest,
    ) -> Result<WebhookSubscription> {
        let url = self.url(&format!("/webhooks/subscriptions/{}", subscription_id));
        let response = self.http().put(&url).json(&req).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn delete_webhook_subscription(&self, subscription_id: &Uuid) -> Result<()> {
        let url = self.url(&format!("/webhooks/subscriptions/{}", subscription_id));
        let response = self.http().delete(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn list_webhook_subscriptions(
        &self,
        req: ListWebhooksRequest,
    ) -> Result<PaginatedResponse<WebhookSubscription>> {
        let mut url = self.url("/webhooks/subscriptions");
        let mut params = vec![];

        if let Some(limit) = req.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = req.offset {
            params.push(format!("offset={}", offset));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.http().get(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn get_webhook_delivery(&self, delivery_id: &Uuid) -> Result<WebhookDelivery> {
        let url = self.url(&format!("/webhooks/deliveries/{}", delivery_id));
        let response = self.http().get(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn list_webhook_deliveries(
        &self,
        req: ListDeliveriesRequest,
    ) -> Result<PaginatedResponse<WebhookDelivery>> {
        let mut url = self.url("/webhooks/deliveries");
        let mut params = vec![];

        if let Some(limit) = req.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = req.offset {
            params.push(format!("offset={}", offset));
        }
        if let Some(subscription_id) = req.subscription_id {
            params.push(format!("subscription_id={}", subscription_id));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self.http().get(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(crate::error::LoyaltyError::Api {
                status: response.status().as_u16(),
                message: response.text().await?,
            })
        }
    }

    pub async fn retry_webhook_delivery(&self, delivery_id: &Uuid) -> Result<WebhookDelivery> {
        let url = self.url(&format!("/webhooks/deliveries/{}/retry", delivery_id));
        let response = self.http().post(&url).send().await?;

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
