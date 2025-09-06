pub mod login;
mod current;

use std::sync::Arc;
use axum::{routing::{get, post}, Router};
use crate::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login::login))
        .route("/current", get(current::current))
}