use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct DatabaseConfig {
    pub db_url: String,
}

impl Settings {
    pub fn new() -> Result<Self, String> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").unwrap_or("cma".to_string());
        Ok(Settings {
            database: DatabaseConfig { db_url },
        })
    }
}
