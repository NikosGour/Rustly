use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct ShortenedUrl {
    pub id: Option<u32>,
    pub full_url: String,
    pub short_url: String,
    pub created_at: DateTime<Utc>,
    pub last_modified_at: Option<DateTime<Utc>>
}

impl ShortenedUrl {
    pub fn new(full_url: String, short_url: String) -> Self{
        ShortenedUrl{
            id: None,
            full_url,
            short_url,
            created_at: Utc::now(),
            last_modified_at: None
        }
    }
}