use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub email: String,
}
