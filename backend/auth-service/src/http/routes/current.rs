use crate::app::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, debug_handler};

#[debug_handler]
pub async fn current(State(_): State<AppState>) -> (StatusCode, Json<String>) {
    (StatusCode::IM_A_TEAPOT, Json::from("hello".to_owned()))
}
