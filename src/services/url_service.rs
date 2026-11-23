use crate::repositories::UrlRepository;
use std::sync::Arc;

pub struct UrlService<'db_pool> {
    pub url_repository: Arc<UrlRepository<'db_pool>>,
}

impl<'db_pool> UrlService<'db_pool> {
    pub fn create_short_url(self, url: String) -> String {
        self.url_repository.insert_url(&url);
        url
    }
}
