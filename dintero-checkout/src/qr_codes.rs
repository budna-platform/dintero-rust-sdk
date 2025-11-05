//! QR code generation for checkout.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeRequest {
    pub session_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeResponse {
    pub qr_code: String,
    pub format: String,
}

impl QrCodeRequest {
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            size: None,
            format: None,
        }
    }

    pub fn builder() -> QrCodeRequestBuilder {
        QrCodeRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct QrCodeRequestBuilder {
    session_id: Option<String>,
    size: Option<u32>,
    format: Option<String>,
}

impl QrCodeRequestBuilder {
    pub fn session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn build(self) -> Result<QrCodeRequest, String> {
        Ok(QrCodeRequest {
            session_id: self.session_id.ok_or("session_id is required")?,
            size: self.size,
            format: self.format,
        })
    }
}
