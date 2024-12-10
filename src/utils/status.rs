#[derive(Debug)]
pub enum ApiStatus {
    Success,
    NotFound,
    BadRequest,
}

impl ApiStatus {
    pub fn as_str(&self) -> String {
        match self {
            ApiStatus::Success => String::from("Success"),
            // ApiStatus::NotFound => "Not Found",
            // ApiStatus::BadRequest => "Bad Request",
            _ => String::from("Not Found"),
        }
    }
}
