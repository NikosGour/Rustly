use crate::repositories::UrlRepository;
use std::sync::Arc;
use rand::distr::SampleString;
use crate::models::ShortenedUrl;

pub struct UrlService {
    pub url_repository: Arc<UrlRepository>,
}

impl UrlService {
    pub async fn create_short_url(&self, url: String) -> String {
        let short_url = {
            let mut rng = rand::rng();
            rand::distr::Alphanumeric.sample_string(&mut rng, 12)
        };
        let short_url_entity = ShortenedUrl::new(url, short_url.clone());
        self.url_repository.insert_url(short_url_entity).await;
        short_url
    }

    pub async fn get_url(&self, short_url: String) -> Option<String> {
        self.url_repository.get_url(short_url).await
    }
}
