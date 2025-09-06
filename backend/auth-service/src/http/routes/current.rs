use crate::app::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::{Json, debug_handler};
use serde_json::{json, Value};
use crate::http::utils::auth;

#[debug_handler]
pub async fn current(
    State(app): State<AppState>,
    headers: HeaderMap,
) -> (StatusCode, Json<Value>) {
    match auth::get_authenticated_user(headers, app.auth_service.as_ref()) {
        Some(user) => (
            StatusCode::OK, 
            Json(serde_json::to_value(&user)
                .expect("unable to serialize user"))
        ),
        None => (
            StatusCode::UNAUTHORIZED, 
            Json(json!({}))
        )
    }
}
