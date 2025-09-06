pub mod routes;

use crate::app::AppState;
use axum::{Router, routing::get};
use hmac::Mac;

pub fn get_router(app_state: AppState) -> Router {
    Router::<AppState>::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api", routes::router())
        .with_state(app_state)
}
