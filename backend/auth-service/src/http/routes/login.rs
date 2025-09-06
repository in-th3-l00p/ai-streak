use crate::app::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, debug_handler};

#[debug_handler]
pub async fn login(State(_): State<AppState>) -> (StatusCode, Json<String>) {
    (StatusCode::CREATED, Json::from("hello".to_owned()))
}
