use crate::errors::database::Error;
use crate::models::ShortenedUrl;
use sqlx::PgPool;
use std::sync::Arc;

pub struct UrlRepository {
    pub pool: Arc<PgPool>,
}

impl UrlRepository {
    pub async fn insert_url(&self, url: ShortenedUrl) -> Option<Error> {
        match sqlx::query!(
            r#"INSERT INTO "url"("full_url", "short_url", "created_at", "last_modified_at")
                        VALUES ($1,$2,now(),NULL)"#,
            url.full_url,
            url.short_url
        )
        .execute(self.pool.as_ref())
        .await
        {
            Ok(_) => None,
            Err(err) => Some(err.into()),
        }
    }

    pub async fn get_url(&self, short_url: String) -> Result<String, Error> {
        match sqlx::query!(
            r#"SELECT "full_url" FROM "url" WHERE "short_url" = $1 AND NOT "disabled""#,
            short_url
        )
        .fetch_one(self.pool.as_ref())
        .await
        {
            Ok(row) => Ok(row.full_url),
            Err(err) => Err(err.into()),
        }
    }
}
