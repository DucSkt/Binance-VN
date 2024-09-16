use crate::api::state::AppState;
use crate::api::user::controller::PaginationParams;
use crate::api::user::dto::CreateUserDto;
use crate::api::user::model::User;
use sqlx::Error;

pub async fn insert_user(app_state: &AppState, dto: CreateUserDto) -> Result<User, Error> {
    let query = "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email";

    sqlx::query_as::<_, User>(query)
        .bind(dto.name)
        .bind(dto.email)
        .fetch_one(&*app_state.db_pool)
        .await
}

pub async fn fetch_users(
    app_state: &AppState,
    params: PaginationParams,
) -> Result<Vec<User>, Error> {
    let skip = params.skip.unwrap_or(0);
    let take = params.take.unwrap_or(10);
    let order_by = params.order_by.unwrap_or("id".to_string());
    let name_filter = params.name.unwrap_or_default();

    let query = format!(
        "SELECT * FROM users WHERE name ILIKE $1 ORDER BY {} LIMIT $2 OFFSET $3",
        order_by
    );

    sqlx::query_as::<_, User>(&query)
        .bind(format!("%{}%", name_filter))
        .bind(take)
        .bind(skip)
        .fetch_all(&*app_state.db_pool)
        .await
}

pub async fn fetch_user(app_state: &AppState, user_id: i32) -> Result<User, Error> {
    let query = "SELECT * FROM users WHERE id = $1";

    sqlx::query_as::<_, User>(query)
        .bind(user_id)
        .fetch_one(&*app_state.db_pool)
        .await
}
