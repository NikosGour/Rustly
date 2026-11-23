use crate::config::state::AppState;
use crate::services::UrlService;
use axum::Router;
use axum::extract::State;
use axum::routing::post;
use std::sync::Arc;

#[derive(Clone)]
pub struct UrlController {
    pub url_service: Arc<UrlService>,
}

impl UrlController {
    pub fn routes() -> Router<AppState> {
        Router::new().route("/", post(Self::create_url))
    }
    async fn create_url(State(state): State<AppState>) -> String {
        let url_service = state.controllers.url_controller.url_service;
        let short_url = url_service.create_short_url("000".to_string()).await;
        short_url
    }
}
