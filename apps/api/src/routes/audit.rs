use std::sync::Arc;

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiResult;
use crate::models::common::PaginationParams;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/audit-log", get(list_audit_log))
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
struct AuditLogEntry {
    id: Uuid,
    team_id: Uuid,
    user_id: Option<Uuid>,
    action: String,
    entity_type: String,
    entity_id: Option<Uuid>,
    old_values: Option<serde_json::Value>,
    new_values: Option<serde_json::Value>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct AuditLogFilters {
    entity_type: Option<String>,
    entity_id: Option<Uuid>,
    user_id: Option<Uuid>,
    action: Option<String>,
}

async fn list_audit_log(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
    Query(filters): Query<AuditLogFilters>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();
    let cursor = pagination.cursor.as_ref().and_then(|c| c.parse::<Uuid>().ok());

    let entries = sqlx::query_as::<_, AuditLogEntry>(
        r#"
        SELECT * FROM audit_log
        WHERE team_id = $1
          AND ($2::text IS NULL OR entity_type = $2)
          AND ($3::uuid IS NULL OR entity_id = $3)
          AND ($4::uuid IS NULL OR user_id = $4)
          AND ($5::text IS NULL OR action = $5)
          AND ($6::uuid IS NULL OR id < $6)
        ORDER BY created_at DESC LIMIT $7
        "#,
    )
    .bind(team_id)
    .bind(&filters.entity_type)
    .bind(filters.entity_id)
    .bind(filters.user_id)
    .bind(&filters.action)
    .bind(cursor)
    .bind(pagination.limit())
    .fetch_all(&state.db)
    .await?;

    let has_more = entries.len() as i64 == pagination.limit();

    Ok(Json(json!({
        "data": entries,
        "meta": { "has_more": has_more, "cursor": entries.last().map(|e| e.id.to_string()) },
        "errors": null,
    })))
}
