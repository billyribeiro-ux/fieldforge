use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::Extension;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::middleware::auth::AuthUser;
use crate::models::time_entry::{StartTimerRequest, StopTimerRequest};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/time-entries/start", post(start_timer))
        .route("/time-entries/{id}/stop", post(stop_timer))
        .route("/jobs/{job_id}/time-entries", get(list_job_time_entries))
        .route("/time-entries/active", get(get_active_timer))
}

async fn start_timer(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(req): Json<StartTimerRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let user_id = auth.id;

    // Check for existing active timer
    let active = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM time_entries WHERE user_id = $1 AND ended_at IS NULL",
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await?;

    if active > 0 {
        return Err(ApiError::Conflict("You already have an active timer running".into()));
    }

    let entry_type = req.entry_type.as_deref().unwrap_or("work");
    let hourly_rate: Option<rust_decimal::Decimal> = sqlx::query_scalar(
        "SELECT hourly_rate FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_one(&state.db)
    .await?;

    let entry = sqlx::query_as::<_, crate::models::time_entry::TimeEntry>(
        r#"
        INSERT INTO time_entries (team_id, job_id, user_id, entry_type, started_at, hourly_rate, latitude_start, longitude_start)
        VALUES ($1, $2, $3, $4::time_entry_type, now(), $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(req.job_id)
    .bind(user_id)
    .bind(entry_type)
    .bind(hourly_rate)
    .bind(req.latitude)
    .bind(req.longitude)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(job_id = %req.job_id, user_id = %user_id, "Timer started");

    Ok(Json(json!({
        "data": entry,
        "meta": null,
        "errors": null,
    })))
}

async fn stop_timer(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<StopTimerRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = auth.id;

    let entry = sqlx::query_as::<_, crate::models::time_entry::TimeEntry>(
        r#"
        UPDATE time_entries SET
            ended_at = now(),
            duration_minutes = EXTRACT(EPOCH FROM (now() - started_at))::int / 60,
            total_cost = CASE
                WHEN hourly_rate IS NOT NULL
                THEN hourly_rate * (EXTRACT(EPOCH FROM (now() - started_at))::numeric / 3600)
                ELSE NULL
            END,
            latitude_end = $3,
            longitude_end = $4,
            notes = $5
        WHERE id = $1 AND user_id = $2 AND ended_at IS NULL
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(user_id)
    .bind(req.latitude)
    .bind(req.longitude)
    .bind(&req.notes)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Active time entry".into()))?;

    // Update job actual duration
    if let Some(duration) = entry.duration_minutes {
        sqlx::query(
            r#"
            UPDATE jobs SET actual_duration_minutes = COALESCE(actual_duration_minutes, 0) + $2
            WHERE id = $1
            "#,
        )
        .bind(entry.job_id)
        .bind(duration)
        .execute(&state.db)
        .await?;
    }

    tracing::info!(entry_id = %id, duration = ?entry.duration_minutes, "Timer stopped");

    Ok(Json(json!({
        "data": entry,
        "meta": null,
        "errors": null,
    })))
}

async fn get_active_timer(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = auth.id;

    let entry = sqlx::query_as::<_, crate::models::time_entry::TimeEntry>(
        "SELECT * FROM time_entries WHERE user_id = $1 AND ended_at IS NULL ORDER BY started_at DESC LIMIT 1",
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?;

    Ok(Json(json!({
        "data": entry,
        "meta": null,
        "errors": null,
    })))
}

async fn list_job_time_entries(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let entries = sqlx::query_as::<_, crate::models::time_entry::TimeEntry>(
        "SELECT * FROM time_entries WHERE job_id = $1 ORDER BY started_at DESC",
    )
    .bind(job_id)
    .fetch_all(&state.db)
    .await?;

    let total_minutes: i64 = entries
        .iter()
        .filter_map(|e| e.duration_minutes.map(|d| d as i64))
        .sum();

    let total_cost: rust_decimal::Decimal = entries
        .iter()
        .filter_map(|e| e.total_cost)
        .sum();

    Ok(Json(json!({
        "data": entries,
        "meta": {
            "total_minutes": total_minutes,
            "total_cost": total_cost,
        },
        "errors": null,
    })))
}
