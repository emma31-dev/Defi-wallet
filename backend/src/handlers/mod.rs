use axum::{self, http::StatusCode};

pub async fn ok() -> (StatusCode, String) {
    (StatusCode::OK, "Ok".into())
}

// pub async fn health() -> ()
