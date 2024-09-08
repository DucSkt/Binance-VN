use crate::api::state::AppState;
use crate::api::user::dto::CreateUserDto;
use crate::api::user::service::create_user;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};

pub async fn create_user_handler(
    State(app_state): State<AppState>,
    Json(dto): Json<CreateUserDto>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Validate DTO
    // if let Err(validation_errors) = dto.validate() {
    //     return Err((StatusCode::BAD_REQUEST, validation_errors.to_string()));
    // }

    // Gọi service để tạo user
    match create_user(&app_state, dto).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
