use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub roles: Vec<String>,
    pub send_invitation: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub roles: Option<Vec<String>>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivity {
    pub id: String,
    pub user_id: String,
    pub activity_type: String,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
}

impl crate::client::AccountsClient {
    pub async fn list_users(&self) -> Result<Vec<User>> {
        self.execute_request(Method::GET, "accounts/current/users", None::<&()>)
            .await
    }

    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        self.execute_request(
            Method::GET,
            &format!("accounts/current/users/{}", user_id),
            None::<&()>,
        )
        .await
    }

    pub async fn create_user(&self, request: &CreateUserRequest) -> Result<User> {
        self.execute_request(Method::POST, "accounts/current/users", Some(request))
            .await
    }

    pub async fn update_user(&self, user_id: &str, request: &UpdateUserRequest) -> Result<User> {
        self.execute_request(
            Method::PUT,
            &format!("accounts/current/users/{}", user_id),
            Some(request),
        )
        .await
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<()> {
        self.execute_request(
            Method::DELETE,
            &format!("accounts/current/users/{}", user_id),
            None::<&()>,
        )
        .await
    }

    pub async fn list_user_activities(&self, user_id: &str) -> Result<Vec<UserActivity>> {
        self.execute_request(
            Method::GET,
            &format!("accounts/current/users/{}/activities", user_id),
            None::<&()>,
        )
        .await
    }

    pub async fn list_permissions(&self) -> Result<Vec<Permission>> {
        self.execute_request(Method::GET, "accounts/current/permissions", None::<&()>)
            .await
    }

    pub async fn list_roles(&self) -> Result<Vec<Role>> {
        self.execute_request(Method::GET, "accounts/current/roles", None::<&()>)
            .await
    }

    pub async fn update_password(&self, request: &UpdatePasswordRequest) -> Result<()> {
        self.execute_request(Method::POST, "accounts/current/password", Some(request))
            .await
    }

    pub async fn reset_password(&self, request: &ResetPasswordRequest) -> Result<()> {
        self.execute_request(Method::POST, "accounts/password/reset", Some(request))
            .await
    }
}
