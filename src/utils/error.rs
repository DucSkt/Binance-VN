use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("forbidden")]
    Forbidden,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    message: "forbidden".to_string(),
                }),
            )
                .into_response(),
            Self::Unknown(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: e.to_string(),
                }),
            )
                .into_response(),
        }
    }
}
