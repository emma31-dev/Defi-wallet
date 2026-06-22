use anyhow::{Context, Result};
use bwb::routes::app;
use bwb::tracing::init_logging;
use tracing::info;
use turso::Builder;
use alloy::rpc::types::Log;
use bwb::structures::AppState;
use bwb::handlers::event::listen;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logging
    init_logging();
    
    let db = Builder::new_local("../db/app.db")
        .build()
        .await
        .context("Failed to load db")?;
    let conn = db
        .connect()
        .context("Failed to load connection with database")?;

    let (tx, rx) = tokio::sync::mpsc::channel::<Log>(100);
    let state = AppState {
        db_conn: conn,
        event_sender: tx,
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    futures::join!(
        async { 
            info!("Server starting...");
            axum::serve(listener, app(state).await)
            .await
            .expect("Server failed to start");
        },
        async {
            info!("Event listener starting...");
            tokio::spawn(async move {
                let _ = listen(rx).await;
            }).await.expect("Failed to spawn contract listener");
        }
    );
    Ok(())
}
