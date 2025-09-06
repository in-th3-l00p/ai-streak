use crate::app::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Json, debug_handler};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[debug_handler]
pub async fn login(
    State(app): State<AppState>,
    Json(body): Json<UserLogin>,
) -> (StatusCode, Json<Value>) {
    let user = app.user_service.login(
        body.username,
        body.password,
    )
        .await;
    match user {
        Ok(user) => match app.auth_service.sign(&user) {
            Ok(jwt) => (
                StatusCode::OK,
                Json(json!({ "jwt": jwt }))
            ),
            Err(err) => {
                tracing::error!(
                    "failed to create jwt for user: {:#?}. error: {}",
                    user,
                    err.to_string()
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "message": "internal server error" }))
                )
            }
        }
        Err(err) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": err.to_string() }))
        ),
    }

}
