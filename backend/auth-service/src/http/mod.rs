pub mod routes;

use crate::app::AppState;
use axum::{routing::get, Router};
use hmac::Mac;
use std::sync::Arc;

pub fn get_router(app_state: AppState) -> Router {
    Router::<Arc<AppState>>::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api", routes::router())
        .with_state(Arc::new(app_state))
}
