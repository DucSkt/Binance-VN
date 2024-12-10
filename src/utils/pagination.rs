use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub limit: Option<i64>,
    pub page: Option<i32>,    // Offset Pagination
    pub cursor: Option<Uuid>, // Keyset Pagination
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub status: String,
    pub data: Vec<T>,
    pub meta: PaginationMeta,
    pub next_cursor: Option<Uuid>, // Keyset Pagination
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub total: Option<i64>, // Total cho Offset Pagination
    pub page: Option<i32>,  // Trang hiện tại
    pub per_page: i32,      // Số lượng bản ghi mỗi trang
}
