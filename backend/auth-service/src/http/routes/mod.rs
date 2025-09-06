pub mod login;
mod current;

use crate::app::AppState;
use axum::{routing::{get, post}, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login::login))
        .route("/current", get(current::current))
}