use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(range(min = 18, max = 100))]
    pub age: i32,
}
