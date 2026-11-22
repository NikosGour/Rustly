use axum::Router;
use axum::routing::get;
use crate::handlers::{health, root, tasks};

pub(crate) fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/task", get(tasks::get_tasks).post(tasks::create_task))
        .route("/task/{id}", get(tasks::get_task_by_id))
}