use anyhow::{Context, Result};
use bwb::handlers::event;
use bwb::routes::app;
use bwb::structures::{AppState, EnvVariables};
use bwb::tracing::init_logging;
use tracing::info;
use turso::Builder;

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

    // let (tx, rx) = tokio::sync::mpsc::channel::<Log>(100);
    let state = AppState { db_conn: conn };
    let config = EnvVariables::new();

    let listener = tokio::net::TcpListener::bind(&config.socket).await?;

    futures::try_join!(
        async {
            info!("Server starting...");
            axum::serve(listener, app(state).await)
                .await
                .context("Server failed to start")
        },
        async {
            info!("Contract deposit Event listener starting...");
            event::deposit_listener(&config)
                .await
                .context("Failed to init listener")
        },
        async {
            info!("Contract withdrawal Event listener starting...");
            event::withdraw_listener(&config)
                .await
                .context("Failed to init listener")
        },
        async {
            info!("Contract transfer Event listener starting...");
            event::transfer_listener(&config)
                .await
                .context("Failed to init listener")
        },
    )?;
    Ok(())
}
