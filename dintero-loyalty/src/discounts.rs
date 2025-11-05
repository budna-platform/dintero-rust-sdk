use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::types::PaginatedResponse;
use crate::error::Result;
use crate::client::LoyaltyClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountRule {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: DiscountType,
    pub amount: Option<i64>,
    pub percentage: Option<f64>,
    pub conditions: Option<serde_json::Value>,
    pub priority: Option<i32>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscountType {
    FixedAmount,
    Percentage,
    FreeShipping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDiscountRuleRequest {
    pub name: String,
    pub description: Option<String>,
    pub discount_type: DiscountType,
    pub amount: Option<i64>,
    pub percentage: Option<f64>,
    pub conditions: Option<serde_json::Value>,
    pub priority: Option<i32>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDiscountRuleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub amount: Option<i64>,
    pub percentage: Option<f64>,
    pub conditions: Option<serde_json::Value>,
    pub priority: Option<i32>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountCampaign {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: Option<String>,
    pub code: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub rule_id: Option<Uuid>,
    pub usage_limit: Option<i32>,
    pub usage_count: Option<i32>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDiscountCampaignRequest {
    pub name: String,
    pub description: Option<String>,
    pub code: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub rule_id: Option<Uuid>,
    pub usage_limit: Option<i32>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDiscountCampaignRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub code: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub usage_limit: Option<i32>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerDiscount {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub campaign_id: Uuid,
    pub assigned_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub used: bool,
    pub used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignDiscountRequest {
    pub campaign_id: Uuid,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default)]
pub struct ListDiscountsRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl LoyaltyClient {
    pub async fn create_discount_rule(&self, req: CreateDiscountRuleRequest) -> Result<DiscountRule> {
        let url = self.url("/discounts/rules");
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

    pub async fn get_discount_rule(&self, rule_id: &Uuid) -> Result<DiscountRule> {
        let url = self.url(&format!("/discounts/rules/{}", rule_id));
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

    pub async fn update_discount_rule(&self, rule_id: &Uuid, req: UpdateDiscountRuleRequest) -> Result<DiscountRule> {
        let url = self.url(&format!("/discounts/rules/{}", rule_id));
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

    pub async fn delete_discount_rule(&self, rule_id: &Uuid) -> Result<()> {
        let url = self.url(&format!("/discounts/rules/{}", rule_id));
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

    pub async fn list_discount_rules(&self, req: ListDiscountsRequest) -> Result<PaginatedResponse<DiscountRule>> {
        let mut url = self.url("/discounts/rules");
        let mut params = vec![];
        
        if let Some(limit) = req.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = req.offset {
            params.push(format!("offset={}", offset));
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

    pub async fn create_discount_campaign(&self, req: CreateDiscountCampaignRequest) -> Result<DiscountCampaign> {
        let url = self.url("/discounts/campaigns");
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

    pub async fn get_discount_campaign(&self, campaign_id: &Uuid) -> Result<DiscountCampaign> {
        let url = self.url(&format!("/discounts/campaigns/{}", campaign_id));
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

    pub async fn update_discount_campaign(&self, campaign_id: &Uuid, req: UpdateDiscountCampaignRequest) -> Result<DiscountCampaign> {
        let url = self.url(&format!("/discounts/campaigns/{}", campaign_id));
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

    pub async fn delete_discount_campaign(&self, campaign_id: &Uuid) -> Result<()> {
        let url = self.url(&format!("/discounts/campaigns/{}", campaign_id));
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

    pub async fn list_discount_campaigns(&self, req: ListDiscountsRequest) -> Result<PaginatedResponse<DiscountCampaign>> {
        let mut url = self.url("/discounts/campaigns");
        let mut params = vec![];
        
        if let Some(limit) = req.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = req.offset {
            params.push(format!("offset={}", offset));
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

    pub async fn assign_discount_to_customer(&self, customer_id: &Uuid, req: AssignDiscountRequest) -> Result<CustomerDiscount> {
        let url = self.url(&format!("/customers/{}/discounts", customer_id));
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

    pub async fn list_customer_discounts(&self, customer_id: &Uuid) -> Result<Vec<CustomerDiscount>> {
        let url = self.url(&format!("/customers/{}/discounts", customer_id));
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
