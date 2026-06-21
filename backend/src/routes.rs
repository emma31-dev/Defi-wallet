use super::handlers::ok;
use axum::{Router, routing::get};
use turso::Builder;

pub async fn app() -> Router {
    let db = Builder::new_local("../db/app.db")
        .build()
        .await
        .expect("Failed to load db");
    let conn = db
        .connect()
        .expect("Failed to load connection with database");

    Router::new().route("/ok", get(ok)).with_state(conn)
}
