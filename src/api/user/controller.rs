use crate::api::state::AppState;
use crate::api::user::dto::CreateUserDto;
use crate::api::user::service::create_user;
use crate::clg;
use crate::utils::error::AppError;
use crate::utils::valid_req::ValidJson;
use axum::{extract::State, http::StatusCode, response::IntoResponse};

pub async fn create_user_handler(
    State(app_state): State<AppState>,
    ValidJson(dto): ValidJson<CreateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    // Gọi service để tạo user
    match create_user(&app_state, dto).await {
        Ok(user) => Ok((StatusCode::CREATED, axum::Json(user))),
        Err(err) => {
            clg!("Create user error", err);
            Err(AppError::DatabaseError(err))
        }
    }
}
