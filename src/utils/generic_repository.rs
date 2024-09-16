use async_trait::async_trait;
use sqlx::{postgres::PgRow, FromRow, PgPool};
use std::sync::Arc;

#[async_trait]
pub trait GenericRepository<T>
where
    for<'a> T: Send + Sync + FromRow<'a, PgRow> + Unpin,
{
    async fn find_many(
        &self,
        table: &str,
        skip: i64,
        take: i64,
        order_by: String,
    ) -> Result<Vec<T>, sqlx::Error>;
    async fn create(&self, table: &str, entity: &T) -> Result<T, sqlx::Error>;
}

pub struct Repository {
    pub db_pool: Arc<PgPool>,
}

#[async_trait]
impl<T> GenericRepository<T> for Repository
where
    for<'a> T: Send + Sync + FromRow<'a, PgRow> + Unpin, // Higher-Ranked Trait Bound is correctly applied
{
    async fn find_many(
        &self,
        table: &str,
        skip: i64,
        take: i64,
        order_by: String,
    ) -> Result<Vec<T>, sqlx::Error> {
        let query = format!(
            "SELECT * FROM {} ORDER BY {} LIMIT $1 OFFSET $2",
            table, order_by
        );
        sqlx::query_as::<_, T>(&query)
            .bind(take)
            .bind(skip)
            .fetch_all(&*self.db_pool)
            .await
    }

    async fn create(&self, table: &str, entity: &T) -> Result<T, sqlx::Error> {
        unimplemented!()
    }
}
