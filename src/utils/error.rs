use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation error")]
    ValidationError(#[from] ValidationErrors),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, details) = match self {
            AppError::ValidationError(errors) => (
                StatusCode::BAD_REQUEST,
                "Validation error".to_string(),
                Some(json!(errors.errors())),
            ),
            AppError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
                Some(json!({ "details": err.to_string() })),
            ),
            AppError::DeserializationError(err) => (
                StatusCode::BAD_REQUEST,
                "Invalid input".to_string(),
                Some(json!({ "details": err })),
            ),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
                None,
            ),
        };

        let body = ErrorResponse {
            code: status.as_u16(),
            message: error_message,
            details,
        };

        (status, Json(body)).into_response()
    }
}
