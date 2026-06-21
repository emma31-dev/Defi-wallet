use super::handlers::ok;
use axum::{Router, routing::get};
use turso::Connection;

pub async fn app(conn: Connection) -> Router {
    Router::new().route("/ok", get(ok)).with_state(conn)
}
