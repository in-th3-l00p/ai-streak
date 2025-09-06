use std::sync::Arc;
use crate::app::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{debug_handler, Json};

#[debug_handler]
pub async fn login(
    State(_): State<Arc<AppState>>,
) -> (StatusCode, Json<String>) {
    (StatusCode::CREATED, Json::from("hello".to_owned()))
}