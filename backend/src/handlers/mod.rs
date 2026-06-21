use axum::{self, http::StatusCode};

pub async fn ok<'a>() -> (StatusCode, &'a str) {
    (StatusCode::OK, "Ok")
}

// pub async fn health() -> ()
