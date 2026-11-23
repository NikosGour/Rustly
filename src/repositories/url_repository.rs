use std::sync::Arc;
use sqlx::PgPool;

pub struct UrlRepository<'db_pool> {
    pub pool: &'db_pool PgPool
}

impl<'a> UrlRepository<'a> {
     pub async fn insert_url(&self, url: &str) {
        sqlx::query!(r#"SELECT 1 as nikos"#).fetch_one(self.pool).await.unwrap();
    }
}