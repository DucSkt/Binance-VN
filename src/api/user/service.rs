use crate::api::state::AppState;
use crate::api::user::dto::CreateUserDto;
use crate::api::user::model::User;
use crate::utils::pagination::PaginationParams;
use chrono::{DateTime, Utc};
use sqlx::Error;
use uuid::Uuid;

pub async fn insert_user(app_state: &AppState, dto: CreateUserDto) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        "
    INSERT INTO users (name, email, age)
    VALUES ($1, $2, $3)
    RETURNING id, name, email, deleted_at, age
    ",
        dto.name,
        dto.email,
        dto.age,
    )
    .fetch_one(&*app_state.db_pool)
    .await
}

pub async fn modify_user(
    app_state: &AppState,
    dto: CreateUserDto,
    user_id: Uuid,
) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        r#"UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email, age, deleted_at"#,
        dto.name,
        dto.email,
        user_id
    )
    .fetch_one(&*app_state.db_pool)
    .await
}

pub async fn fetch_users(
    app_state: &AppState,
    params: &PaginationParams,
) -> Result<(Vec<User>, Option<i64>, Option<Uuid>), Error> {
    let limit = params.limit.unwrap_or(10);
    if let Some(mut cursor) = params.cursor {
        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id > $1 ORDER BY id LIMIT $2",
            cursor,
            limit as i64
        )
        .fetch_all(&*app_state.db_pool)
        .await?;

        let next_cursor = users.last().map(|u| u.id);
        Ok((users, None, next_cursor))
    } else {
        let page = params.page.unwrap_or(1);
        let offset = (page - 1) as i64 * limit;
        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users ORDER BY id LIMIT $1 OFFSET $2",
            limit,
            offset as i64
        )
        .fetch_all(&*app_state.db_pool)
        .await?;
        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(&*app_state.db_pool)
            .await?;
        Ok((users, Some(total), None))
    }
}

pub async fn fetch_user(app_state: &AppState, user_id: Uuid) -> Result<User, Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&*app_state.db_pool)
        .await
}

pub async fn remove_user(app_state: &AppState, user_id: Uuid) -> Result<User, Error> {
    let now: DateTime<Utc> = Utc::now();
    sqlx::query_as!(
        User,
        "UPDATE users SET deleted_at = $1 WHERE id = $2 RETURNING id, name, email, age, deleted_at",
        now,
        user_id
    )
    .fetch_one(&*app_state.db_pool)
    .await
}
