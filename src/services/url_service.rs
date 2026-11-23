use crate::repositories::UrlRepository;
use std::sync::Arc;

pub struct UrlService {
    pub url_repository: Arc<UrlRepository>,
}

impl UrlService {
    pub async fn create_short_url(&self, url: String) -> String {
        self.url_repository.insert_url(&url).await;
        url
    }
}
