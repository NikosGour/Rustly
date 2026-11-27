use crate::errors::database::Error;
use crate::models::ShortenedUrl;
use crate::repositories::UrlRepository;
use rand::distr::SampleString;
use std::sync::Arc;

pub struct UrlService {
    pub url_repository: Arc<UrlRepository>,
}

impl UrlService {
    pub async fn create_short_url(&self, url: String) -> Result<String, Error> {
        let short_url = {
            let mut rng = rand::rng();
            rand::distr::Alphanumeric.sample_string(&mut rng, 12)
        };
        let short_url_entity = ShortenedUrl::new(url, short_url.clone());
        self.url_repository.insert_url(short_url_entity).await?;
        Ok(short_url)
    }

    pub async fn get_url(&self, short_url: String) -> Result<String, Error> {
        self.url_repository.get_url(short_url).await
    }
}
