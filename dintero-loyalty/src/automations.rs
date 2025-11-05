//! Automation rules for loyalty programs.

use crate::client::LoyaltyClient;
use crate::error::Result;
use crate::types::PaginatedResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub requirement: AutomationRequirement,
    pub limitation: Option<AutomationLimitation>,
    pub actions: Vec<AutomationAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRequirement {
    pub automation_from: Option<DateTime<Utc>>,
    pub automation_to: Option<DateTime<Utc>>,
    pub events: Vec<String>,
    pub filter: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationLimitation {
    pub automation_repeat: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAutomationRuleRequest {
    pub name: String,
    pub description: Option<String>,
    pub requirement: AutomationRequirement,
    pub limitation: Option<AutomationLimitation>,
    pub actions: Vec<AutomationAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAutomationRuleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub requirement: Option<AutomationRequirement>,
    pub limitation: Option<AutomationLimitation>,
    pub actions: Option<Vec<AutomationAction>>,
}

#[derive(Debug, Clone, Default)]
pub struct ListAutomationRulesRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl LoyaltyClient {
    pub async fn create_automation_rule(
        &self,
        req: CreateAutomationRuleRequest,
    ) -> Result<AutomationRule> {
        let url = self.url("/automations/rules");
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

    pub async fn get_automation_rule(&self, rule_id: &Uuid) -> Result<AutomationRule> {
        let url = self.url(&format!("/automations/rules/{}", rule_id));
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

    pub async fn update_automation_rule(
        &self,
        rule_id: &Uuid,
        req: UpdateAutomationRuleRequest,
    ) -> Result<AutomationRule> {
        let url = self.url(&format!("/automations/rules/{}", rule_id));
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

    pub async fn delete_automation_rule(&self, rule_id: &Uuid) -> Result<()> {
        let url = self.url(&format!("/automations/rules/{}", rule_id));
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

    pub async fn list_automation_rules(
        &self,
        req: ListAutomationRulesRequest,
    ) -> Result<PaginatedResponse<AutomationRule>> {
        let mut url = self.url("/automations/rules");
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
}
