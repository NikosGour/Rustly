use crate::models::ShortenedUrl::ShortenedUrl;
use sqlx::PgPool;
use std::sync::Arc;

pub struct UrlRepository {
    pub pool: Arc<PgPool>,
}

impl UrlRepository {
    pub async fn insert_url(&self, url: ShortenedUrl) {
        sqlx::query!(
            r#"INSERT INTO "url"("full_url", "short_url", "created_at", "last_modified_at")
                        VALUES ($1,$2,now(),NULL)"#,
            url.full_url,
            url.short_url
        )
        .execute(self.pool.as_ref())
        .await
        .unwrap();
    }
}
