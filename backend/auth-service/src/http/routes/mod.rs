mod current;
pub mod login;

use crate::app::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login::login))
        .route("/current", get(current::current))
}
