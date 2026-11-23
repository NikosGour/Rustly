use std::sync::Arc;
use axum::Router;
use axum::http::StatusCode;
use axum::routing::post;
use crate::services::UrlService;

pub struct UrlController<'db_pool> {
    pub url_service: Arc<UrlService<'db_pool>>
}

impl UrlController<'_> {
    pub fn routes(self) -> Router {
        Router::new().route("/", post(self.create_url()))
    }

    fn create_url(self) -> (StatusCode, String) {
        (StatusCode::CREATED, "done".into())
    }
}
