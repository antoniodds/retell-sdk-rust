//! Error types returned by the Retell SDK.

use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

/// Convenient `Result` alias for SDK operations.
pub type Result<T> = std::result::Result<T, Error>;

/// All errors that can occur while using the Retell client.
#[derive(Debug, Error)]
pub enum Error {
    /// The request could not be built (invalid URL, header, etc.).
    #[error("failed to build request: {0}")]
    RequestBuild(String),

    /// A network / transport error from `reqwest`.
    #[error("HTTP transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// The API returned a non-success status with a JSON error body.
    #[error("{status}: {message}")]
    Api {
        status: StatusCode,
        message: String,
        body: Option<ApiErrorBody>,
    },

    /// The API returned a non-success status with a non-JSON body.
    #[error("HTTP {status}: {body}")]
    Http { status: StatusCode, body: String },

    /// Response JSON could not be deserialized into the expected type.
    #[error("failed to decode response: {0}")]
    Decode(#[source] serde_json::Error),

    /// Request JSON could not be serialized.
    #[error("failed to encode request: {0}")]
    Encode(#[source] serde_json::Error),
}

impl Error {
    /// HTTP status code when this is an API / HTTP error.
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Api { status, .. } | Self::Http { status, .. } => Some(*status),
            _ => None,
        }
    }

    pub fn is_unauthorized(&self) -> bool {
        self.status() == Some(StatusCode::UNAUTHORIZED)
    }

    pub fn is_not_found(&self) -> bool {
        self.status() == Some(StatusCode::NOT_FOUND)
    }

    pub fn is_rate_limited(&self) -> bool {
        self.status() == Some(StatusCode::TOO_MANY_REQUESTS)
    }

    pub fn is_payment_required(&self) -> bool {
        self.status() == Some(StatusCode::PAYMENT_REQUIRED)
    }
}

/// JSON error payload returned by Retell (`status` + `message`).
#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorBody {
    pub status: Option<String>,
    pub message: Option<String>,
}

impl Error {
    pub(crate) async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status();
        let bytes = match response.bytes().await {
            Ok(b) => b,
            Err(e) => return Self::Transport(e),
        };

        if let Ok(body) = serde_json::from_slice::<ApiErrorBody>(&bytes) {
            let message = body
                .message
                .clone()
                .unwrap_or_else(|| status.canonical_reason().unwrap_or("error").to_string());
            return Self::Api {
                status,
                message,
                body: Some(body),
            };
        }

        let body = String::from_utf8_lossy(&bytes).into_owned();
        Self::Http { status, body }
    }
}
