use crate::api::state::AppState;
use crate::api::user::dto::CreateUserDto;
use crate::api::user::service::{fetch_user, fetch_users, insert_user};
use crate::utils::error::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationMeta, PaginationParams};
use crate::utils::valid_req::ValidJson;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn create_user(
    State(app_state): State<AppState>,
    ValidJson(dto): ValidJson<CreateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    match insert_user(&app_state, dto).await {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(err) => Err(AppError::DatabaseError(err)),
    }
}

pub async fn list_users(
    State(app_state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    match fetch_users(&app_state, &params).await {
        Ok((users, total, next_cursor)) => {
            let response = PaginatedResponse {
                status: "success".to_string(),
                data: users,
                meta: PaginationMeta {
                    total,
                    page: params.page,
                    per_page: params.limit.unwrap_or(10) as i32,
                },
                next_cursor,
            };
            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => Err(AppError::DatabaseError(err)),
    }
}

pub async fn list_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    match fetch_user(&app_state, user_id).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(err) => Err(AppError::DatabaseError(err)),
        // Err(_) => Err(AppError::DatabaseError("User not found".into())),
    }
}
