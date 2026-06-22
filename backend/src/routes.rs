use super::handlers::ok;
use axum::{Router, routing::get};
use turso::Connection;

<<<<<<< HEAD
pub async fn app(conn: Connection) -> Router {
=======
pub async fn app() -> Router {
    let db = Builder::new_local("../db/app.db")
        .build()
        .await
        .expect("Failed to load db");
    let conn = db
        .connect()
        .expect("Failed to load connection with database");

>>>>>>> 9d81effe7ec8b72ba80f52040ce7773d3274cadf
    Router::new().route("/ok", get(ok)).with_state(conn)
}
