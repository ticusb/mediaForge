use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

/// Application-wide error type with proper HTTP status mapping
#[derive(Debug)]
pub enum AppError {
    // Client errors (4xx)
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    PayloadTooLarge(String),
    QuotaExceeded(String),
    UnprocessableEntity(String),

    // Server errors (5xx)
    Internal(String),
    ServiceUnavailable(String),
    
    // External errors
    Database(sqlx::Error),
    Io(std::io::Error),
    ImageProcessing(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Self::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            Self::NotFound(msg) => write!(f, "Not Found: {}", msg),
            Self::Conflict(msg) => write!(f, "Conflict: {}", msg),
            Self::PayloadTooLarge(msg) => write!(f, "Payload Too Large: {}", msg),
            Self::QuotaExceeded(msg) => write!(f, "Quota Exceeded: {}", msg),
            Self::UnprocessableEntity(msg) => write!(f, "Unprocessable Entity: {}", msg),
            Self::Internal(msg) => write!(f, "Internal Server Error: {}", msg),
            Self::ServiceUnavailable(msg) => write!(f, "Service Unavailable: {}", msg),
            Self::Database(err) => write!(f, "Database Error: {}", err),
            Self::Io(err) => write!(f, "IO Error: {}", err),
            Self::ImageProcessing(msg) => write!(f, "Image Processing Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Conversions from other error types
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", err);
        match err {
            sqlx::Error::RowNotFound => Self::NotFound("Resource not found".to_string()),
            sqlx::Error::PoolTimedOut => {
                Self::ServiceUnavailable("Database connection pool timeout".to_string())
            }
            _ => Self::Database(err),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        tracing::error!("IO error: {:?}", err);
        Self::Io(err)
    }
}

impl From<image::ImageError> for AppError {
    fn from(err: image::ImageError) -> Self {
        tracing::error!("Image processing error: {:?}", err);
        Self::ImageProcessing(err.to_string())
    }
}

impl From<crate::services::processing::ProcessingError> for AppError {
    fn from(err: crate::services::processing::ProcessingError) -> Self {
        tracing::error!("Processing error: {:?}", err);
        Self::ImageProcessing(err.to_string())
    }
}

// Convert AppError to HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match &self {
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            Self::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone()),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            Self::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone()),
            Self::PayloadTooLarge(msg) => {
                (StatusCode::PAYLOAD_TOO_LARGE, "PAYLOAD_TOO_LARGE", msg.clone())
            }
            Self::QuotaExceeded(msg) => {
                (StatusCode::TOO_MANY_REQUESTS, "QUOTA_EXCEEDED", msg.clone())
            }
            Self::UnprocessableEntity(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "UNPROCESSABLE_ENTITY",
                msg.clone(),
            ),
            Self::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                msg.clone(),
            ),
            Self::ServiceUnavailable(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE",
                msg.clone(),
            ),
            Self::Database(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                format!("A database error occurred: {}", err),
            ),
            Self::Io(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "IO_ERROR",
                format!("An IO error occurred: {}", err),
            ),
            Self::ImageProcessing(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "PROCESSING_ERROR",
                msg.clone(),
            ),
        };

        // Log error details
        if status.is_server_error() {
            tracing::error!("Server error: {:?}", self);
        } else {
            tracing::warn!("Client error: {:?}", self);
        }

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": message,
            }
        }));

        (status, body).into_response()
    }
}

/// Convenience type alias for Results
pub type Result<T> = std::result::Result<T, AppError>;