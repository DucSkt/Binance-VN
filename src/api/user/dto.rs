use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub name: Option<String>,
    pub email: Option<String>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UpdateUserDto {
//     pub name: Option<String>,
//     pub email: Option<String>,
// }
