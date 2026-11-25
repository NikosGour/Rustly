use crate::config::state::AppState;
use crate::services::UrlService;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;
use crate::dtos::CreateUrlReq;

#[derive(Clone)]
pub struct UrlController {
    pub url_service: Arc<UrlService>,
}

impl UrlController {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/", post(Self::create_url))
            .route("/{short_url}", get(Self::get_url))
    }
    async fn create_url(
        State(state): State<AppState>,
        Json(req): Json<CreateUrlReq>,
    ) -> (StatusCode, String) {
        let url_service = state.controllers.url_controller.url_service;
        let short_url = url_service.create_short_url(req.full_url).await;
        (StatusCode::CREATED, short_url)
    }

    async fn get_url(State(state): State<AppState>, Path(short_url): Path<String>) -> Result<Redirect, StatusCode> {
        let url_service = state.controllers.url_controller.url_service;
        let Ok(full_url) = url_service.get_url(short_url).await else {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        };

        Ok(Redirect::temporary(&("https://".to_string() + &full_url)))
    }
}