use crate::config::settings::DatabaseConfig;
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use tracing::info;
use tracing_subscriber::fmt::format;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(config: DatabaseConfig) -> Result<DbPool, sqlx::Error> {
    info!("database_url. {}", &config.db_url);
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.db_url)
        .await
        .expect("Failed to create pool.");
    Ok(pool)
}
