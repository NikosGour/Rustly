use crate::config::state::AppState;
use axum::Router;
use axum::routing::get;

pub(crate) fn app(app_state: AppState) -> Router {
    Router::new()
        // .route("/", get(root))
        // .route("/health", get(health))
        .nest("/url", app_state.controllers.url_controller.routes())
}