use crate::api::state::AppState;
use crate::api::user::dto::CreateUserDto;
use sqlx::Error;

pub async fn create_user(app_state: &AppState, dto: CreateUserDto) -> Result<(), Error> {
    let query = "INSERT INTO users (name, email) VALUES ($1, $2)";

    // Tương tác với database bằng cách sử dụng db_pool từ AppState
    sqlx::query(query)
        .bind(&dto.name)
        .bind(&dto.email)
        .execute(&*app_state.db_pool)
        .await?;

    Ok(())
}
