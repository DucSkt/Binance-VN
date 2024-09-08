use crate::api::state::AppState;
use crate::api::user::dto::CreateUserDto;
use crate::api::user::service::create_user;
use crate::utils::error::AppError;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;

pub async fn create_user_handler(
    State(app_state): State<AppState>,
    Json(dto): Json<CreateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    // Validate DTO
    if let Err(validation_errors) = dto.validate() {
        return Err(AppError::ValidationError(validation_errors));
    }

    // Gọi service để tạo user
    match create_user(&app_state, dto).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(err) => Err(AppError::DatabaseError(err)),
    }
}
