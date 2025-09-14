use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)] // Allow unused variants for future features
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Couchbase API error: {message} (status: {status})")]
    CouchbaseApi { message: String, status: u16 },

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Config(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            AppError::Http(msg) => (StatusCode::BAD_GATEWAY, msg.to_string()),
            AppError::Json(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            AppError::Io(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            AppError::Utf8(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            AppError::CouchbaseApi { message, status } => {
                let status_code = StatusCode::from_u16(status).unwrap_or(StatusCode::BAD_GATEWAY);
                (status_code, message)
            }
            AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
