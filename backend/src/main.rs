use anyhow::{Result, anyhow};
use bwb::routes::app;
use bwb::tracing::init_logging;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logging
    init_logging();
    info!("Server starting...");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app().await)
        .await
        .map(|a| {
            info!("Server has started.");
            a
        })
        .map_err(|e| anyhow!("Server failed to start, {:?}", e))
}
