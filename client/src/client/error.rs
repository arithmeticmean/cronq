use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    /// Failed to initialize HTTP client
    #[error("Client initialization failed: {0}")]
    Init(String),

    /// Network-level errors
    #[error("Network error: {0}")]
    RequestError(#[from] reqwest_middleware::Error),

    #[error("Network error: {0}")]
    ResponseError(#[from] reqwest::Error),

    /// Server returned 4xx/5xx with error details
    #[error("API error (status: {status}): {body}")]
    ApiError { status: StatusCode, body: String },

    /// Invalid URLs
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// Serialization/Deserlization error
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}
