use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::{get, patch};
use axum::{Json, Router};
use axum::Extension;
use serde_json::json;
use uuid::Uuid;

use crate::db::repository;
use crate::errors::{ApiError, ApiResult};
use crate::middleware::auth::AuthUser;
use crate::models::common::PaginationParams;
use crate::models::job::{CreateJobRequest, JobFilters, JobStatusTransition, UpdateJobRequest};
use crate::services::job_service;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/jobs", get(list_jobs).post(create_job))
        .route("/jobs/{id}", get(get_job).patch(update_job))
        .route("/jobs/{id}/status", patch(transition_status))
}

async fn create_job(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(req): Json<CreateJobRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let job = repository::create_job(&state.db, team_id, &req).await?;

    tracing::info!(job_id = %job.id, "Job created: {}", job.title);

    Ok(Json(json!({
        "data": job,
        "meta": null,
        "errors": null,
    })))
}

async fn list_jobs(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Query(pagination): Query<PaginationParams>,
    Query(filters): Query<JobFilters>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let cursor = pagination.cursor.as_ref().and_then(|c| c.parse::<Uuid>().ok());

    let jobs = repository::list_jobs(&state.db, team_id, &filters, pagination.limit(), cursor).await?;
    let has_more = jobs.len() as i64 == pagination.limit();

    Ok(Json(json!({
        "data": jobs,
        "meta": {
            "has_more": has_more,
            "cursor": jobs.last().map(|j| j.id.to_string()),
        },
        "errors": null,
    })))
}

async fn get_job(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let job = repository::get_job(&state.db, team_id, id).await?;

    Ok(Json(json!({
        "data": job,
        "meta": null,
        "errors": null,
    })))
}

async fn update_job(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateJobRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    // Verify job exists and belongs to team
    let _existing = repository::get_job(&state.db, team_id, id).await?;

    let updated = sqlx::query_as::<_, crate::models::job::Job>(
        r#"
        UPDATE jobs SET
            title = COALESCE($3, title),
            description = COALESCE($4, description),
            priority = COALESCE($5::job_priority, priority),
            assigned_to = COALESCE($6, assigned_to),
            scheduled_date = COALESCE($7, scheduled_date),
            scheduled_start_time = COALESCE($8, scheduled_start_time),
            scheduled_end_time = COALESCE($9, scheduled_end_time),
            estimated_duration_minutes = COALESCE($10, estimated_duration_minutes),
            internal_notes = COALESCE($11, internal_notes),
            version = version + 1
        WHERE id = $1 AND team_id = $2 AND deleted_at IS NULL
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.priority)
    .bind(req.assigned_to)
    .bind(req.scheduled_date)
    .bind(req.scheduled_start_time)
    .bind(req.scheduled_end_time)
    .bind(req.estimated_duration_minutes)
    .bind(&req.internal_notes)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({
        "data": updated,
        "meta": null,
        "errors": null,
    })))
}

async fn transition_status(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<JobStatusTransition>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let user_id = auth.id;

    let job = repository::get_job(&state.db, team_id, id).await?;

    // Validate FSM transition
    if !job_service::is_valid_transition(&job.status, &req.status) {
        return Err(ApiError::BadRequest(format!(
            "Invalid status transition: {} → {}",
            job.status, req.status
        )));
    }

    let updated = repository::update_job_status(
        &state.db,
        team_id,
        id,
        &req.status,
        user_id,
        req.latitude,
        req.longitude,
        req.note.as_deref(),
    )
    .await?;

    // Trigger side effects
    let side_effects = job_service::get_transition_side_effects(&job.status, &req.status);
    for effect in &side_effects {
        tracing::info!(job_id = %id, effect = %effect, "Triggering side effect");
        // TODO: dispatch to background job queue
    }

    Ok(Json(json!({
        "data": updated,
        "meta": {
            "side_effects": side_effects,
        },
        "errors": null,
    })))
}
