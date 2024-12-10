use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
    pub deleted_at: Option<DateTime<Utc>>,
}
