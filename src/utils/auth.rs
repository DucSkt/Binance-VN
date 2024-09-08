// use anyhow::Result;
// use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
// use serde::{Deserialize, Serialize};
//
// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     sub: String,
//     exp: usize,
// }
//
// const SECRET: &[u8] = b"your-secret";
//
// pub fn create_token(user_id: &str) -> Result<String> {
//     let my_claims = Claims {
//         sub: user_id.to_owned(),
//         exp: 10000000000,
//     };
//     let token = encode(
//         &Header::default(),
//         &my_claims,
//         &EncodingKey::from_secret(SECRET),
//     )?;
//     Ok(token)
// }
//
// pub fn validate_token(token: &str) -> Result<Claims> {
//     let token_data = decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(SECRET),
//         &Validation::default(),
//     )?;
//     Ok(token_data.claims)
// }
