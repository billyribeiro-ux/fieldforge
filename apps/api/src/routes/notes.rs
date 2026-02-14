use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use axum::Extension;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::middleware::auth::AuthUser;
use crate::models::note::CreateNoteRequest;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/jobs/{job_id}/notes", get(list_job_notes).post(create_note))
        .route("/customers/{customer_id}/notes", get(list_customer_notes))
        .route("/notes/{id}", get(get_note).delete(delete_note))
}

async fn create_note(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(job_id): Path<Uuid>,
    Json(req): Json<CreateNoteRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let user_id = auth.id;

    let note_type = req.note_type.as_deref().unwrap_or("text");
    let is_internal = req.is_internal.unwrap_or(false);

    let note = sqlx::query_as::<_, crate::models::note::Note>(
        r#"
        INSERT INTO notes (team_id, job_id, customer_id, author_id, content, note_type, is_internal)
        VALUES ($1, $2, $3, $4, $5, $6::note_type, $7)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(job_id)
    .bind(req.customer_id)
    .bind(user_id)
    .bind(&req.content)
    .bind(note_type)
    .bind(is_internal)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(note_id = %note.id, job_id = %job_id, "Note created");

    Ok(Json(json!({
        "data": note,
        "meta": null,
        "errors": null,
    })))
}

async fn list_job_notes(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let notes = sqlx::query_as::<_, crate::models::note::Note>(
        "SELECT * FROM notes WHERE job_id = $1 ORDER BY created_at DESC",
    )
    .bind(job_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": notes,
        "meta": { "total": notes.len() },
        "errors": null,
    })))
}

async fn list_customer_notes(
    State(state): State<Arc<AppState>>,
    Path(customer_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let notes = sqlx::query_as::<_, crate::models::note::Note>(
        "SELECT * FROM notes WHERE customer_id = $1 ORDER BY created_at DESC",
    )
    .bind(customer_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": notes,
        "meta": { "total": notes.len() },
        "errors": null,
    })))
}

async fn get_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let note = sqlx::query_as::<_, crate::models::note::Note>(
        "SELECT * FROM notes WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Note".into()))?;

    Ok(Json(json!({
        "data": note,
        "meta": null,
        "errors": null,
    })))
}

async fn delete_note(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    sqlx::query("DELETE FROM notes WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Note deleted" },
        "errors": null,
    })))
}
