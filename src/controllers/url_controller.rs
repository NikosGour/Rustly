use crate::config::state::AppState;
use crate::services::UrlService;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use std::sync::Arc;
use axum::http::StatusCode;
use crate::models::CreateUrlReq::CreateUrlReq;

#[derive(Clone)]
pub struct UrlController {
    pub url_service: Arc<UrlService>,
}

impl UrlController {
    pub fn routes() -> Router<AppState> {
        Router::new().route("/", post(Self::create_url))
    }
    async fn create_url(State(state): State<AppState>, Json(req): Json<CreateUrlReq>) -> (StatusCode, String) {
        let url_service = state.controllers.url_controller.url_service;
        let short_url = url_service.create_short_url(req.full_url).await;
        (StatusCode::CREATED, short_url)
    }
}
