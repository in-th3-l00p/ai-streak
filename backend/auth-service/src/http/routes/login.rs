use crate::http::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{debug_handler, Json};

#[debug_handler]
pub async fn login(
    State(_): State<AppState>,
) -> (StatusCode, Json<String>) {
    (StatusCode::CREATED, Json::from("hello".to_owned()))
}