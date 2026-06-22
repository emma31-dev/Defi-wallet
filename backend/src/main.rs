use anyhow::{Context, Result};
use bwb::routes::app;
use bwb::tracing::init_logging;
use tracing::info;
use turso::Builder;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logging
    init_logging();
    info!("Server starting...");

    let db = Builder::new_local("../db/app.db")
        .build()
        .await
        .context("Failed to load db")?;
    let conn = db
        .connect()
        .context("Failed to load connection with database")?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app(conn).await)
        .await
        .context("Server failed to start")
}
