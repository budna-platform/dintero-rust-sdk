use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::types::PaginatedResponse;
use crate::error::Result;
use crate::client::LoyaltyClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCatalog {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductCatalogRequest {
    pub name: String,
    pub description: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProductCatalogRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductItem {
    pub id: Uuid,
    pub catalog_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub currency: String,
    pub tax_rate: Option<f64>,
    pub stock: Option<i32>,
    pub active: bool,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductItemRequest {
    pub catalog_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub currency: String,
    pub tax_rate: Option<f64>,
    pub stock: Option<i32>,
    pub active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProductItemRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i64>,
    pub tax_rate: Option<f64>,
    pub stock: Option<i32>,
    pub active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
pub struct ListProductsRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub catalog_id: Option<Uuid>,
}

impl LoyaltyClient {
    pub async fn create_product_catalog(&self, req: CreateProductCatalogRequest) -> Result<ProductCatalog> {
        let url = self.url("/products/catalogs");
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

    pub async fn get_product_catalog(&self, catalog_id: &Uuid) -> Result<ProductCatalog> {
        let url = self.url(&format!("/products/catalogs/{}", catalog_id));
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

    pub async fn update_product_catalog(&self, catalog_id: &Uuid, req: UpdateProductCatalogRequest) -> Result<ProductCatalog> {
        let url = self.url(&format!("/products/catalogs/{}", catalog_id));
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

    pub async fn delete_product_catalog(&self, catalog_id: &Uuid) -> Result<()> {
        let url = self.url(&format!("/products/catalogs/{}", catalog_id));
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

    pub async fn list_product_catalogs(&self, req: ListProductsRequest) -> Result<PaginatedResponse<ProductCatalog>> {
        let mut url = self.url("/products/catalogs");
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

    pub async fn create_product_item(&self, req: CreateProductItemRequest) -> Result<ProductItem> {
        let url = self.url("/products/items");
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

    pub async fn get_product_item(&self, item_id: &Uuid) -> Result<ProductItem> {
        let url = self.url(&format!("/products/items/{}", item_id));
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

    pub async fn update_product_item(&self, item_id: &Uuid, req: UpdateProductItemRequest) -> Result<ProductItem> {
        let url = self.url(&format!("/products/items/{}", item_id));
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

    pub async fn delete_product_item(&self, item_id: &Uuid) -> Result<()> {
        let url = self.url(&format!("/products/items/{}", item_id));
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

    pub async fn list_product_items(&self, req: ListProductsRequest) -> Result<PaginatedResponse<ProductItem>> {
        let mut url = self.url("/products/items");
        let mut params = vec![];
        
        if let Some(limit) = req.limit {
            params.push(format!("limit={}", limit));
        }
        if let Some(offset) = req.offset {
            params.push(format!("offset={}", offset));
        }
        if let Some(catalog_id) = req.catalog_id {
            params.push(format!("catalog_id={}", catalog_id));
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
