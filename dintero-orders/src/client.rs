use crate::authorizations::*;
use crate::cancellations::*;
use crate::captures::*;
use crate::comments::*;
use crate::drafts::*;
use crate::events::*;
use crate::orders::*;
use crate::refunds::*;
use crate::sessions::*;
use async_trait::async_trait;

pub type Result<T> = std::result::Result<T, OrdersError>;

#[derive(Debug, thiserror::Error)]
pub enum OrdersError {
    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Client error: {0}")]
    Client(String),
}

impl From<serde_json::Error> for OrdersError {
    fn from(err: serde_json::Error) -> Self {
        OrdersError::Serialization(err.to_string())
    }
}

#[async_trait]
pub trait OrdersOperations: Send + Sync {
    async fn create_order(&self, request: CreateOrderRequest) -> Result<Order>;
    async fn get_order(&self, order_id: &str) -> Result<Order>;
    async fn update_order(&self, order_id: &str, request: CreateOrderRequest) -> Result<Order>;
    async fn list_orders(&self, params: ListOrdersParams) -> Result<OrderListResponse>;
    async fn close_order(&self, order_id: &str) -> Result<Order>;
    async fn open_order(&self, order_id: &str) -> Result<Order>;

    async fn create_draft_order(&self, request: CreateDraftOrderRequest) -> Result<DraftOrder>;
    async fn get_draft_order(&self, draft_id: &str) -> Result<DraftOrder>;
    async fn update_draft_order(
        &self,
        draft_id: &str,
        request: CreateDraftOrderRequest,
    ) -> Result<DraftOrder>;
    async fn list_draft_orders(&self) -> Result<DraftOrderListResponse>;
    async fn complete_draft_order(&self, draft_id: &str) -> Result<Order>;
    async fn add_draft_order_item(
        &self,
        draft_id: &str,
        request: AddDraftOrderItemRequest,
    ) -> Result<DraftOrderItem>;
    async fn update_draft_order_item(
        &self,
        draft_id: &str,
        line_id: &str,
        request: UpdateDraftOrderItemRequest,
    ) -> Result<DraftOrderItem>;
    async fn delete_draft_order_item(&self, draft_id: &str, line_id: &str) -> Result<()>;

    async fn create_authorization(
        &self,
        order_id: &str,
        request: CreateAuthorizationRequest,
    ) -> Result<Authorization>;
    async fn get_authorization(&self, order_id: &str, auth_id: &str) -> Result<Authorization>;
    async fn list_authorizations(&self, order_id: &str) -> Result<AuthorizationListResponse>;

    async fn create_capture(
        &self,
        order_id: &str,
        request: CreateCaptureRequest,
    ) -> Result<Capture>;
    async fn get_capture(&self, order_id: &str, capture_id: &str) -> Result<Capture>;
    async fn list_captures(&self, order_id: &str) -> Result<CaptureListResponse>;

    async fn create_refund(&self, order_id: &str, request: CreateRefundRequest) -> Result<Refund>;
    async fn get_refund(&self, order_id: &str, refund_id: &str) -> Result<Refund>;
    async fn list_refunds(&self, order_id: &str) -> Result<RefundListResponse>;

    async fn create_cancellation(
        &self,
        order_id: &str,
        request: CreateCancellationRequest,
    ) -> Result<Cancellation>;
    async fn get_cancellation(&self, order_id: &str, cancellation_id: &str)
        -> Result<Cancellation>;
    async fn list_cancellations(&self, order_id: &str) -> Result<CancellationListResponse>;

    async fn create_comment(
        &self,
        order_id: &str,
        request: CreateCommentRequest,
    ) -> Result<Comment>;
    async fn delete_comment(&self, order_id: &str, comment_id: &str) -> Result<()>;

    async fn get_events(&self, order_id: &str) -> Result<EventListResponse>;
    async fn create_event(&self, order_id: &str, request: CreateEventRequest)
        -> Result<OrderEvent>;

    async fn create_order_session(
        &self,
        order_id: &str,
        request: CreateOrderSessionRequest,
    ) -> Result<OrderSession>;
    async fn get_order_session(&self, order_id: &str, session_id: &str) -> Result<OrderSession>;
    async fn list_order_sessions(&self, order_id: &str) -> Result<OrderSessionListResponse>;

    async fn list_customer_orders(&self, customer_id: &str) -> Result<OrderListResponse>;
    async fn get_customer_order(&self, customer_id: &str, order_id: &str) -> Result<Order>;

    async fn list_store_orders(&self, store_id: &str) -> Result<OrderListResponse>;
    async fn get_store_order(&self, store_id: &str, order_id: &str) -> Result<Order>;
}

#[async_trait]
pub trait OrdersAdapter: Send + Sync {
    async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T>;
    async fn post_json<T: serde::de::DeserializeOwned, B: serde::Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T>;
    async fn put_json<T: serde::de::DeserializeOwned, B: serde::Serialize + Send + Sync>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T>;
    async fn delete(&self, path: &str) -> Result<()>;
}

pub struct OrdersClient<A: OrdersAdapter> {
    adapter: A,
    account_id: String,
}

impl<A: OrdersAdapter> OrdersClient<A> {
    pub fn new(adapter: A, account_id: String) -> Self {
        Self {
            adapter,
            account_id,
        }
    }
}

#[async_trait]
impl<A: OrdersAdapter> OrdersOperations for OrdersClient<A> {
    async fn create_order(&self, request: CreateOrderRequest) -> Result<Order> {
        let path = format!("accounts/{}/orders", self.account_id);
        self.adapter.post_json(&path, &request).await
    }

    async fn get_order(&self, order_id: &str) -> Result<Order> {
        let path = format!("accounts/{}/orders/{}", self.account_id, order_id);
        self.adapter.get_json(&path).await
    }

    async fn update_order(&self, order_id: &str, request: CreateOrderRequest) -> Result<Order> {
        let path = format!("accounts/{}/orders/{}", self.account_id, order_id);
        self.adapter.put_json(&path, &request).await
    }

    async fn list_orders(&self, params: ListOrdersParams) -> Result<OrderListResponse> {
        let mut path = format!("accounts/{}/orders", self.account_id);

        let mut query_params = Vec::new();
        if let Some(limit) = params.limit {
            query_params.push(format!("limit={}", limit));
        }
        if let Some(token) = params.page_token {
            query_params.push(format!("page_token={}", token));
        }
        if let Some(status) = params.status {
            query_params.push(format!("status={:?}", status));
        }
        if let Some(reference) = params.merchant_reference {
            query_params.push(format!("merchant_reference={}", reference));
        }

        if !query_params.is_empty() {
            path.push('?');
            path.push_str(&query_params.join("&"));
        }

        self.adapter.get_json(&path).await
    }

    async fn close_order(&self, order_id: &str) -> Result<Order> {
        let path = format!("accounts/{}/orders/{}/close", self.account_id, order_id);
        self.adapter.post_json(&path, &serde_json::json!({})).await
    }

    async fn open_order(&self, order_id: &str) -> Result<Order> {
        let path = format!("accounts/{}/orders/{}/open", self.account_id, order_id);
        self.adapter.post_json(&path, &serde_json::json!({})).await
    }

    async fn create_draft_order(&self, request: CreateDraftOrderRequest) -> Result<DraftOrder> {
        let path = format!("accounts/{}/draft_orders", self.account_id);
        self.adapter.post_json(&path, &request).await
    }

    async fn get_draft_order(&self, draft_id: &str) -> Result<DraftOrder> {
        let path = format!("accounts/{}/draft_orders/{}", self.account_id, draft_id);
        self.adapter.get_json(&path).await
    }

    async fn update_draft_order(
        &self,
        draft_id: &str,
        request: CreateDraftOrderRequest,
    ) -> Result<DraftOrder> {
        let path = format!("accounts/{}/draft_orders/{}", self.account_id, draft_id);
        self.adapter.put_json(&path, &request).await
    }

    async fn list_draft_orders(&self) -> Result<DraftOrderListResponse> {
        let path = format!("accounts/{}/draft_orders", self.account_id);
        self.adapter.get_json(&path).await
    }

    async fn complete_draft_order(&self, draft_id: &str) -> Result<Order> {
        let path = format!(
            "accounts/{}/draft_orders/{}/complete",
            self.account_id, draft_id
        );
        self.adapter.put_json(&path, &serde_json::json!({})).await
    }

    async fn add_draft_order_item(
        &self,
        draft_id: &str,
        request: AddDraftOrderItemRequest,
    ) -> Result<DraftOrderItem> {
        let path = format!(
            "accounts/{}/draft_orders/{}/items",
            self.account_id, draft_id
        );
        self.adapter.post_json(&path, &request).await
    }

    async fn update_draft_order_item(
        &self,
        draft_id: &str,
        line_id: &str,
        request: UpdateDraftOrderItemRequest,
    ) -> Result<DraftOrderItem> {
        let path = format!(
            "accounts/{}/draft_orders/{}/items/{}",
            self.account_id, draft_id, line_id
        );
        self.adapter.put_json(&path, &request).await
    }

    async fn delete_draft_order_item(&self, draft_id: &str, line_id: &str) -> Result<()> {
        let path = format!(
            "accounts/{}/draft_orders/{}/items/{}",
            self.account_id, draft_id, line_id
        );
        self.adapter.delete(&path).await
    }

    async fn create_authorization(
        &self,
        order_id: &str,
        request: CreateAuthorizationRequest,
    ) -> Result<Authorization> {
        let path = format!(
            "accounts/{}/orders/{}/authorization",
            self.account_id, order_id
        );
        self.adapter.post_json(&path, &request).await
    }

    async fn get_authorization(&self, order_id: &str, auth_id: &str) -> Result<Authorization> {
        let path = format!(
            "accounts/{}/orders/{}/authorization/{}",
            self.account_id, order_id, auth_id
        );
        self.adapter.get_json(&path).await
    }

    async fn list_authorizations(&self, order_id: &str) -> Result<AuthorizationListResponse> {
        let path = format!(
            "accounts/{}/orders/{}/authorizations",
            self.account_id, order_id
        );
        self.adapter.get_json(&path).await
    }

    async fn create_capture(
        &self,
        order_id: &str,
        request: CreateCaptureRequest,
    ) -> Result<Capture> {
        let path = format!("accounts/{}/orders/{}/capture", self.account_id, order_id);
        self.adapter.post_json(&path, &request).await
    }

    async fn get_capture(&self, order_id: &str, capture_id: &str) -> Result<Capture> {
        let path = format!(
            "accounts/{}/orders/{}/captures/{}",
            self.account_id, order_id, capture_id
        );
        self.adapter.get_json(&path).await
    }

    async fn list_captures(&self, order_id: &str) -> Result<CaptureListResponse> {
        let path = format!("accounts/{}/orders/{}/captures", self.account_id, order_id);
        self.adapter.get_json(&path).await
    }

    async fn create_refund(&self, order_id: &str, request: CreateRefundRequest) -> Result<Refund> {
        let path = format!("accounts/{}/orders/{}/refunds", self.account_id, order_id);
        self.adapter.post_json(&path, &request).await
    }

    async fn get_refund(&self, order_id: &str, refund_id: &str) -> Result<Refund> {
        let path = format!(
            "accounts/{}/orders/{}/refunds/{}",
            self.account_id, order_id, refund_id
        );
        self.adapter.get_json(&path).await
    }

    async fn list_refunds(&self, order_id: &str) -> Result<RefundListResponse> {
        let path = format!("accounts/{}/orders/{}/refunds", self.account_id, order_id);
        self.adapter.get_json(&path).await
    }

    async fn create_cancellation(
        &self,
        order_id: &str,
        request: CreateCancellationRequest,
    ) -> Result<Cancellation> {
        let path = format!(
            "accounts/{}/orders/{}/cancellation",
            self.account_id, order_id
        );
        self.adapter.post_json(&path, &request).await
    }

    async fn get_cancellation(
        &self,
        order_id: &str,
        cancellation_id: &str,
    ) -> Result<Cancellation> {
        let path = format!(
            "accounts/{}/orders/{}/cancellation/{}",
            self.account_id, order_id, cancellation_id
        );
        self.adapter.get_json(&path).await
    }

    async fn list_cancellations(&self, order_id: &str) -> Result<CancellationListResponse> {
        let path = format!(
            "accounts/{}/orders/{}/cancellations",
            self.account_id, order_id
        );
        self.adapter.get_json(&path).await
    }

    async fn create_comment(
        &self,
        order_id: &str,
        request: CreateCommentRequest,
    ) -> Result<Comment> {
        let path = format!("accounts/{}/orders/{}/comments", self.account_id, order_id);
        self.adapter.post_json(&path, &request).await
    }

    async fn delete_comment(&self, order_id: &str, comment_id: &str) -> Result<()> {
        let path = format!(
            "accounts/{}/orders/{}/comments/{}",
            self.account_id, order_id, comment_id
        );
        self.adapter.delete(&path).await
    }

    async fn get_events(&self, order_id: &str) -> Result<EventListResponse> {
        let path = format!("accounts/{}/orders/{}/events", self.account_id, order_id);
        self.adapter.get_json(&path).await
    }

    async fn create_event(
        &self,
        order_id: &str,
        request: CreateEventRequest,
    ) -> Result<OrderEvent> {
        let path = format!("accounts/{}/orders/{}/events", self.account_id, order_id);
        self.adapter.post_json(&path, &request).await
    }

    async fn create_order_session(
        &self,
        order_id: &str,
        request: CreateOrderSessionRequest,
    ) -> Result<OrderSession> {
        let path = format!("accounts/{}/orders/{}/session", self.account_id, order_id);
        self.adapter.post_json(&path, &request).await
    }

    async fn get_order_session(&self, order_id: &str, session_id: &str) -> Result<OrderSession> {
        let path = format!(
            "accounts/{}/orders/{}/sessions/{}",
            self.account_id, order_id, session_id
        );
        self.adapter.get_json(&path).await
    }

    async fn list_order_sessions(&self, order_id: &str) -> Result<OrderSessionListResponse> {
        let path = format!("accounts/{}/orders/{}/sessions", self.account_id, order_id);
        self.adapter.get_json(&path).await
    }

    async fn list_customer_orders(&self, customer_id: &str) -> Result<OrderListResponse> {
        let path = format!(
            "accounts/{}/customers/{}/orders",
            self.account_id, customer_id
        );
        self.adapter.get_json(&path).await
    }

    async fn get_customer_order(&self, customer_id: &str, order_id: &str) -> Result<Order> {
        let path = format!(
            "accounts/{}/customers/{}/orders/{}",
            self.account_id, customer_id, order_id
        );
        self.adapter.get_json(&path).await
    }

    async fn list_store_orders(&self, store_id: &str) -> Result<OrderListResponse> {
        let path = format!("accounts/{}/stores/{}/orders", self.account_id, store_id);
        self.adapter.get_json(&path).await
    }

    async fn get_store_order(&self, store_id: &str, order_id: &str) -> Result<Order> {
        let path = format!(
            "accounts/{}/stores/{}/orders/{}",
            self.account_id, store_id, order_id
        );
        self.adapter.get_json(&path).await
    }
}
