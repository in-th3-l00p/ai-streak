use std::sync::Arc;
use crate::app::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{debug_handler, Json};

#[debug_handler]
pub async fn current(
    State(_): State<Arc<AppState>>,
) -> (StatusCode, Json<String>) {
    (StatusCode::IM_A_TEAPOT, Json::from("hello".to_owned()))
}
