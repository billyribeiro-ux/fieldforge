use axum::extract::State;
use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health_check))
        .route("/health/ready", get(readiness_check))
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "fieldforge-api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

async fn readiness_check(
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    let db_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();

    let redis_ok = {
        let mut conn = state.redis.clone();
        redis::cmd("PING")
            .query_async::<_, String>(&mut conn)
            .await
            .is_ok()
    };

    let all_ok = db_ok && redis_ok;

    Json(json!({
        "status": if all_ok { "ready" } else { "degraded" },
        "service": "fieldforge-api",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "checks": {
            "database": if db_ok { "connected" } else { "disconnected" },
            "redis": if redis_ok { "connected" } else { "disconnected" },
        }
    }))
}
