use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PaginationParams {
    pub limit: Option<i64>,
    pub page: Option<i32>,   // Offset Pagination
    pub cursor: Option<i32>, // Keyset Pagination
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub status: String,
    pub data: Vec<T>,
    pub meta: PaginationMeta,
    pub next_cursor: Option<i32>, // Keyset Pagination
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub total: Option<i64>, // Total cho Offset Pagination
    pub page: Option<i32>,  // Trang hiện tại
    pub per_page: i32,      // Số lượng bản ghi mỗi trang
}

// Bạn có thể thêm các hàm và helper khác ở đây
