use axum::http::StatusCode;

pub async fn ok<'a>() -> (StatusCode, &'a str) {
    (StatusCode::OK, "Ok")
}

// pub async fn health() -> ()
