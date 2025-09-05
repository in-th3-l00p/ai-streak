use crate::http::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{debug_handler, Json};

#[debug_handler]
pub async fn current(
    State(_): State<AppState>,
) -> (StatusCode, Json<String>) {
    (StatusCode::IM_A_TEAPOT, Json::from("hello".to_owned()))
}
