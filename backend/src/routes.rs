use super::handlers;
use crate::structures::AppState;
use axum::{Router, routing::get};

pub async fn router(state: AppState) -> Router {
    Router::new()
        .route("/ok", get(handlers::ok))
        .route("/rpc_health", get(handlers::rpc_health))
        .route("/db_health", get(handlers::db_health))
        .with_state(state)
}
