use crate::api::state::AppState;
use crate::api::user::dto::CreateUserDto;
use crate::api::user::service::create_user;
use crate::clg;
use crate::utils::error::AppError;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::from_value;
use validator::Validate;

pub async fn create_user_handler(
    State(app_state): State<AppState>,
    payload: axum::extract::Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    // Deserialize and validate DTO
    let dto: CreateUserDto =
        from_value(payload.0).map_err(|e| AppError::DeserializationError(e.to_string()))?;
    dto.validate()?;

    // Gọi service để tạo user
    match create_user(&app_state, dto).await {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(err) => {
            clg!("Create user error", err);
            Err(AppError::DatabaseError(err))
        }
    }
}
