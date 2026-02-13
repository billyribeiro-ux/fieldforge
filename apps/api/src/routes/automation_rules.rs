use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::models::automation_rule::{AutomationRule, CreateAutomationRuleRequest};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/automation-rules", get(list_rules).post(create_rule))
        .route(
            "/automation-rules/{id}",
            get(get_rule).patch(update_rule).delete(delete_rule),
        )
        .route("/automation-rules/{id}/toggle", axum::routing::post(toggle_rule))
}

async fn list_rules(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let rules = sqlx::query_as::<_, AutomationRule>(
        "SELECT * FROM automation_rules WHERE team_id = $1 ORDER BY created_at DESC",
    )
    .bind(team_id)
    .fetch_all(&*state.db)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(json!({ "data": rules, "meta": null, "errors": null })))
}

async fn create_rule(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateAutomationRuleRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let rule = sqlx::query_as::<_, AutomationRule>(
        r#"INSERT INTO automation_rules (team_id, name, trigger_event, conditions, actions, delay_minutes)
           VALUES ($1, $2, $3, $4, $5, $6)
           RETURNING *"#,
    )
    .bind(team_id)
    .bind(&req.name)
    .bind(&req.trigger_event)
    .bind(req.conditions.as_ref().unwrap_or(&serde_json::json!({})))
    .bind(&req.actions)
    .bind(req.delay_minutes.unwrap_or(0))
    .fetch_one(&*state.db)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(json!({ "data": rule, "meta": null, "errors": null })))
}

async fn get_rule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let rule = sqlx::query_as::<_, AutomationRule>(
        "SELECT * FROM automation_rules WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&*state.db)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?
    .ok_or_else(|| ApiError::NotFound("Automation rule not found".into()))?;

    Ok(Json(json!({ "data": rule, "meta": null, "errors": null })))
}

async fn update_rule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let rule = sqlx::query_as::<_, AutomationRule>(
        r#"UPDATE automation_rules
           SET name = COALESCE($3, name),
               trigger_event = COALESCE($4, trigger_event),
               actions = COALESCE($5, actions),
               delay_minutes = COALESCE($6, delay_minutes),
               updated_at = NOW()
           WHERE id = $1 AND team_id = $2
           RETURNING *"#,
    )
    .bind(id)
    .bind(team_id)
    .bind(req.get("name").and_then(|v| v.as_str()))
    .bind(req.get("trigger_event").and_then(|v| v.as_str()))
    .bind(req.get("actions"))
    .bind(req.get("delay_minutes").and_then(|v| v.as_i64()).map(|v| v as i32))
    .fetch_one(&*state.db)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(json!({ "data": rule, "meta": null, "errors": null })))
}

async fn delete_rule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    sqlx::query("DELETE FROM automation_rules WHERE id = $1 AND team_id = $2")
        .bind(id)
        .bind(team_id)
        .execute(&*state.db)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(json!({ "data": null, "meta": null, "errors": null })))
}

async fn toggle_rule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let rule = sqlx::query_as::<_, AutomationRule>(
        r#"UPDATE automation_rules
           SET is_active = NOT is_active, updated_at = NOW()
           WHERE id = $1 AND team_id = $2
           RETURNING *"#,
    )
    .bind(id)
    .bind(team_id)
    .fetch_one(&*state.db)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(json!({ "data": rule, "meta": null, "errors": null })))
}
