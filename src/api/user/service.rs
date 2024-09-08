use crate::api::state::AppState;
use crate::api::user::dto::CreateUserDto;
use crate::api::user::model::User;
use sqlx::Error;

pub async fn create_user(app_state: &AppState, dto: CreateUserDto) -> Result<User, Error> {
    let query = "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email";

    sqlx::query_as::<_, User>(query)
        .bind(dto.name)
        .bind(dto.email)
        .fetch_one(&*app_state.db_pool)
        .await
}
