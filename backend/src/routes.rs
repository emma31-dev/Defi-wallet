use super::handlers::ok;
use crate::structures::AppState;
use axum::{Router, routing::get};

pub async fn app(state: AppState) -> Router {
    Router::new().route("/ok", get(ok)).with_state(state)
}
