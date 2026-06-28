use anyhow::{Context, Result};
use bwb::handlers::event;
use bwb::routes::router;
use bwb::structures::{AppState, EnvVariables};
use bwb::tracing::init_logging;
use tracing::info;
use turso::Builder;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logging
    init_logging();
    let config = EnvVariables::new();

    let db = Builder::new_local(&config.database_url)
        .build()
        .await
        .context("Failed to load db")?;
    let conn = db
        .connect()
        .context("Failed to load connection with database")?;

    // let (tx, rx) = tokio::sync::mpsc::channel::<Log>(100);
    let state = AppState { db_conn: conn, config: config.clone() };

    let listener = tokio::net::TcpListener::bind(&config.socket).await?;

    futures::try_join!(
        async {
            info!("Server starting...");
            axum::serve(listener, router(state).await)
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
