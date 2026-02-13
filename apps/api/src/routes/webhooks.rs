use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/webhooks", get(list_webhooks).post(create_webhook))
        .route("/webhooks/{id}", get(get_webhook).delete(delete_webhook))
        .route("/webhooks/{id}/test", axum::routing::post(test_webhook))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
struct Webhook {
    id: Uuid,
    team_id: Uuid,
    url: String,
    secret: String,
    events: Vec<String>,
    is_active: bool,
    last_triggered_at: Option<chrono::DateTime<chrono::Utc>>,
    last_status_code: Option<i32>,
    failure_count: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct CreateWebhookRequest {
    url: String,
    events: Vec<String>,
}

async fn list_webhooks(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let webhooks = sqlx::query_as::<_, Webhook>(
        "SELECT * FROM webhooks WHERE team_id = $1 ORDER BY created_at DESC",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": webhooks,
        "meta": { "total": webhooks.len() },
        "errors": null,
    })))
}

async fn create_webhook(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateWebhookRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    // Generate a random secret for HMAC signing
    let secret = format!("whsec_{}", Uuid::new_v4().to_string().replace('-', ""));

    let webhook = sqlx::query_as::<_, Webhook>(
        r#"
        INSERT INTO webhooks (team_id, url, secret, events)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(&req.url)
    .bind(&secret)
    .bind(&req.events)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(webhook_id = %webhook.id, url = %req.url, "Webhook created");

    Ok(Json(json!({
        "data": webhook,
        "meta": null,
        "errors": null,
    })))
}

async fn get_webhook(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let webhook = sqlx::query_as::<_, Webhook>(
        "SELECT * FROM webhooks WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Webhook".into()))?;

    Ok(Json(json!({
        "data": webhook,
        "meta": null,
        "errors": null,
    })))
}

async fn delete_webhook(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    sqlx::query("DELETE FROM webhooks WHERE id = $1 AND team_id = $2")
        .bind(id)
        .bind(team_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Webhook deleted" },
        "errors": null,
    })))
}

async fn test_webhook(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    // TODO: Send a test payload to the webhook URL
    tracing::info!(webhook_id = %id, "Webhook test triggered");

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Test webhook sent" },
        "errors": null,
    })))
}
