pub mod event;

use alloy::providers::{Provider, ProviderBuilder};
use axum::{extract::State, http::StatusCode};
use crate::structures::AppState;

pub async fn ok<'a>() -> (StatusCode, &'a str) {
    (StatusCode::OK, "Ok")
}

pub async fn rpc_health<'b>(State(state): State<AppState>) -> Result<(StatusCode, &'b str), (StatusCode, &'b str)> {
    let ws_url = &state.config.rpc_endpoint;
    let provider = ProviderBuilder::new()
        .connect(ws_url)
        .await
        .expect("Failed to connect to provider");

    match provider.get_block_number().await {
        Ok(_) => Ok((StatusCode::OK, "Healthy")),
        Err(_) => Err((StatusCode::SERVICE_UNAVAILABLE, "Unhealthy"))
    }
}

pub async fn db_health<'c>(State(state): State<AppState>) -> Result<(StatusCode, &'c str), (StatusCode, &'c str)> {
    match state.db_conn.prepare("SELECT 1 FROM TABLE").await {
        Ok(_) => Ok((StatusCode::OK, "Healthy")),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Unhealthy"))
    }
}