use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use axum::Extension;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::middleware::auth::AuthUser;
use crate::models::tag::Tag;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/tags", get(list_tags).post(create_tag))
        .route("/tags/{id}", get(get_tag).delete(delete_tag))
}

async fn list_tags(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let tags = sqlx::query_as::<_, Tag>(
        "SELECT * FROM tags WHERE team_id = $1 ORDER BY name ASC",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({ "data": tags, "meta": null, "errors": null })))
}

async fn create_tag(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(req): Json<CreateTagRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let tag = sqlx::query_as::<_, Tag>(
        "INSERT INTO tags (team_id, name, color) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(team_id)
    .bind(&req.name)
    .bind(&req.color)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({ "data": tag, "meta": null, "errors": null })))
}

async fn get_tag(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let tag = sqlx::query_as::<_, Tag>(
        "SELECT * FROM tags WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Tag".into()))?;

    Ok(Json(json!({ "data": tag, "meta": null, "errors": null })))
}

async fn delete_tag(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    sqlx::query("DELETE FROM tags WHERE id = $1 AND team_id = $2")
        .bind(id)
        .bind(team_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({ "data": null, "meta": null, "errors": null })))
}

#[derive(Debug, serde::Deserialize)]
struct CreateTagRequest {
    name: String,
    color: Option<String>,
}
