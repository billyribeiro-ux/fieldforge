use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiResult;
use crate::models::common::PaginationParams;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/notifications", get(list_notifications))
        .route("/notifications/{id}/read", post(mark_read))
        .route("/notifications/read-all", post(mark_all_read))
        .route("/notifications/unread-count", get(unread_count))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
struct Notification {
    id: Uuid,
    team_id: Uuid,
    user_id: Uuid,
    notification_type: String,
    title: String,
    body: String,
    data: Option<serde_json::Value>,
    read_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct NotificationFilters {
    unread_only: Option<bool>,
}

async fn list_notifications(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
    Query(filters): Query<NotificationFilters>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::nil();
    let cursor = pagination.cursor.as_ref().and_then(|c| c.parse::<Uuid>().ok());
    let unread_only = filters.unread_only.unwrap_or(false);

    let notifications = sqlx::query_as::<_, Notification>(
        r#"
        SELECT * FROM notifications
        WHERE user_id = $1
          AND ($2::bool = false OR read_at IS NULL)
          AND ($3::uuid IS NULL OR id < $3)
        ORDER BY created_at DESC LIMIT $4
        "#,
    )
    .bind(user_id)
    .bind(unread_only)
    .bind(cursor)
    .bind(pagination.limit())
    .fetch_all(&state.db)
    .await?;

    let has_more = notifications.len() as i64 == pagination.limit();

    Ok(Json(json!({
        "data": notifications,
        "meta": { "has_more": has_more },
        "errors": null,
    })))
}

async fn mark_read(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::nil();

    sqlx::query(
        "UPDATE notifications SET read_at = now() WHERE id = $1 AND user_id = $2 AND read_at IS NULL",
    )
    .bind(id)
    .bind(user_id)
    .execute(&state.db)
    .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Notification marked as read" },
        "errors": null,
    })))
}

async fn mark_all_read(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::nil();

    let result = sqlx::query(
        "UPDATE notifications SET read_at = now() WHERE user_id = $1 AND read_at IS NULL",
    )
    .bind(user_id)
    .execute(&state.db)
    .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": format!("{} notifications marked as read", result.rows_affected()) },
        "errors": null,
    })))
}

async fn unread_count(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::nil();

    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND read_at IS NULL",
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({
        "data": { "count": count },
        "meta": null,
        "errors": null,
    })))
}
