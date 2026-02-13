use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "fieldforge-api",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
