use axum::{http::StatusCode, response::IntoResponse};

pub mod api;

pub async fn root() -> impl IntoResponse {
    StatusCode::FORBIDDEN
}
