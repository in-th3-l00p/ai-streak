use std::sync::Arc;
use axum::{debug_handler, Json};
use axum::extract::State;
use axum::http::StatusCode;
use crate::http::AppState;

#[debug_handler]
pub async fn login(
    State(_): State<AppState>,
) -> (StatusCode, Json<String>) {
    (StatusCode::CREATED, Json::from("hello".to_owned()))
}