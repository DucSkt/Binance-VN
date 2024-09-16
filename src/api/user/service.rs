use crate::api::state::AppState;
use crate::api::user::dto::CreateUserDto;
use crate::api::user::model::User;
use crate::utils::pagination::PaginationParams;
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
    params: &PaginationParams,
) -> Result<(Vec<User>, Option<i64>, Option<i32>), Error> {
    let limit = params.limit.unwrap_or(10);
    if let Some(cursor) = params.cursor {
        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id > $1 ORDER BY id LIMIT $2",
            cursor,
            limit
        )
        .fetch_all(&app_state.db_pool)
        .await?;

        let next_cursor = users.last().map(|u| u.id);
        Ok((users, None, next_cursor))
    } else {
        // Offset pagination logic
        let page = params.page.unwrap_or(1);
        let offset = (page - 1) as i64 * limit;

        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users ORDER BY id LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(&app_state.db_pool)
        .await?;

        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(&app_state.db_pool)
            .await?;

        Ok((users, Some(total), None)) // Không cần `next_cursor` cho offset
    }
}

pub async fn fetch_user(app_state: &AppState, user_id: i32) -> Result<User, Error> {
    let query = "SELECT * FROM users WHERE id = $1";

    sqlx::query_as::<_, User>(query)
        .bind(user_id)
        .fetch_one(&*app_state.db_pool)
        .await
}
