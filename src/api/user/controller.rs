use crate::api::state::AppState;
use crate::api::user::dto::CreateUserDto;
use crate::api::user::service::{fetch_user, fetch_users, insert_user};
use crate::utils::error::AppError;
use crate::utils::valid_req::ValidJson;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub skip: Option<i64>,
    pub take: Option<i64>,
    pub order_by: Option<String>,
    pub name: Option<String>,
}

pub async fn create_user(
    State(app_state): State<AppState>,
    ValidJson(dto): ValidJson<CreateUserDto>,
) -> Result<impl IntoResponse, AppError> {
    match insert_user(&app_state, dto).await {
        Ok(user) => Ok((StatusCode::CREATED, axum::Json(user))),
        Err(err) => Err(AppError::DatabaseError(err)),
    }
}

pub async fn list_users(
    State(app_state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    match fetch_users(&app_state, params).await {
        Ok(users) => Ok((StatusCode::OK, axum::Json(users))),
        Err(err) => Err(AppError::DatabaseError(err)),
    }
}
pub async fn list_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    match fetch_user(&app_state, user_id).await {
        Ok(user) => Ok((StatusCode::OK, axum::Json(user))),
        Err(err) => Err(AppError::DatabaseError(err)),
        // Err(_) => Err(AppError::DatabaseError("User not found".into())),
    }
}
