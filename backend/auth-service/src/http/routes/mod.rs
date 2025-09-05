pub mod login;
mod current;

use axum::{routing::{get, post}, Router};
use crate::http::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login::login))
        .route("/current", get(current::current))
}