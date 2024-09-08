use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
}
