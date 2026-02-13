use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::models::recurring_rule::{CreateRecurringRuleRequest, RecurringRule};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/recurring-rules", get(list_rules).post(create_rule))
        .route(
            "/recurring-rules/{id}",
            get(get_rule).delete(delete_rule),
        )
        .route("/recurring-rules/{id}/toggle", axum::routing::post(toggle_rule))
        .route("/recurring-rules/{id}/generate", axum::routing::post(generate_next))
}

async fn list_rules(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let rules = sqlx::query_as::<_, RecurringRule>(
        "SELECT * FROM recurring_rules WHERE team_id = $1 ORDER BY next_occurrence ASC NULLS LAST",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({ "data": rules, "meta": null, "errors": null })))
}

async fn create_rule(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateRecurringRuleRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let rule = sqlx::query_as::<_, RecurringRule>(
        r#"INSERT INTO recurring_rules (team_id, customer_id, property_id, title, description, frequency, interval_value, day_of_week, day_of_month, start_date, end_date, estimated_duration_minutes, assigned_to, job_type, priority, auto_schedule, advance_days, next_occurrence)
           VALUES ($1, $2, $3, $4, $5, $6::recurring_frequency, $7, $8, $9, $10, $11, $12, $13, $14, $15::job_priority, $16, $17, $10)
           RETURNING *"#,
    )
    .bind(team_id)
    .bind(req.customer_id)
    .bind(req.property_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.frequency)
    .bind(req.interval_value.unwrap_or(1))
    .bind(req.day_of_week)
    .bind(req.day_of_month)
    .bind(req.start_date)
    .bind(req.end_date)
    .bind(req.estimated_duration_minutes)
    .bind(req.assigned_to)
    .bind(&req.job_type)
    .bind(req.priority.as_deref().unwrap_or("normal"))
    .bind(req.auto_schedule.unwrap_or(true))
    .bind(req.advance_days.unwrap_or(7))
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({ "data": rule, "meta": null, "errors": null })))
}

async fn get_rule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let rule = sqlx::query_as::<_, RecurringRule>(
        "SELECT * FROM recurring_rules WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Recurring rule".into()))?;

    Ok(Json(json!({ "data": rule, "meta": null, "errors": null })))
}

async fn delete_rule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    sqlx::query("DELETE FROM recurring_rules WHERE id = $1 AND team_id = $2")
        .bind(id)
        .bind(team_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({ "data": null, "meta": null, "errors": null })))
}

async fn toggle_rule(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let rule = sqlx::query_as::<_, RecurringRule>(
        r#"UPDATE recurring_rules SET is_active = NOT is_active, updated_at = NOW()
           WHERE id = $1 AND team_id = $2 RETURNING *"#,
    )
    .bind(id)
    .bind(team_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({ "data": rule, "meta": null, "errors": null })))
}

async fn generate_next(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let rule = sqlx::query_as::<_, RecurringRule>(
        "SELECT * FROM recurring_rules WHERE id = $1 AND team_id = $2 AND is_active = true",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Recurring rule".into()))?;

    // Create a job from the recurring rule
    let job = sqlx::query_as::<_, crate::models::job::Job>(
        r#"INSERT INTO jobs (team_id, customer_id, property_id, title, description, status, priority, job_type, estimated_duration_minutes, assigned_to, scheduled_start)
           VALUES ($1, $2, $3, $4, $5, 'scheduled'::job_status, $6::job_priority, $7, $8, $9, $10::date)
           RETURNING *"#,
    )
    .bind(team_id)
    .bind(rule.customer_id)
    .bind(rule.property_id)
    .bind(&rule.title)
    .bind(&rule.description)
    .bind(&rule.priority)
    .bind(&rule.job_type)
    .bind(rule.estimated_duration_minutes)
    .bind(rule.assigned_to)
    .bind(rule.next_occurrence)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(rule_id = %id, job_id = %job.id, "Generated job from recurring rule");

    Ok(Json(json!({ "data": job, "meta": null, "errors": null })))
}
