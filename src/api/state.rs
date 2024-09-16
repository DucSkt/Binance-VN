use crate::config::database::create_pool;
use crate::config::settings::Settings;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<PgPool>,
}

impl AppState {
    pub async fn new(settings: Settings) -> Self {
        let db_pool = create_pool(settings.database)
            .await
            .expect("Failed to create pool");
        AppState {
            db_pool: Arc::new(db_pool),
        }
    }
}
