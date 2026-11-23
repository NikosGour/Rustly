use sqlx::PgPool;
use std::sync::Arc;

pub struct UrlRepository {
    pub pool: Arc<PgPool>,
}

impl UrlRepository {
    pub async fn insert_url(&self, url: &str) {
        sqlx::query!(
            r#"INSERT INTO "url"("full_url", "short_url", "created_at", "last_modified_at")
                        VALUES ('nikos',$1,now(),NULL)"#,
            url
        )
        .execute(self.pool.as_ref())
        .await
        .unwrap();
    }
}
