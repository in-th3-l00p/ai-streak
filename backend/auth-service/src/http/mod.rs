pub mod routes;
mod utils;

use crate::app::AppState;
use axum::{Router, routing::get};

pub fn get_router(app_state: AppState) -> Router {
    Router::<AppState>::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api/auth", routes::router())
        .with_state(app_state)
}
