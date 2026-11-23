use crate::config::state::AppState;
use axum::Router;
use crate::controllers::UrlController;

pub(crate) fn app(app_state: AppState) -> Router {
    Router::new()
        // .route("/", get(root))
        // .route("/health", get(health))
        .nest("/url", UrlController::routes())
        .with_state(app_state)
}