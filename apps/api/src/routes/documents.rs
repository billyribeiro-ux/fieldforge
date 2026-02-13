use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::models::document::{Document, Signature};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/documents", get(list_documents).post(create_document))
        .route("/documents/{id}", get(get_document).delete(delete_document))
        .route("/signatures", get(list_signatures).post(create_signature))
}

#[derive(Debug, serde::Deserialize)]
struct DocumentFilter {
    entity_type: Option<String>,
    entity_id: Option<Uuid>,
}

async fn list_documents(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<DocumentFilter>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let docs = if let (Some(entity_type), Some(entity_id)) = (&filter.entity_type, filter.entity_id) {
        sqlx::query_as::<_, Document>(
            "SELECT * FROM documents WHERE team_id = $1 AND entity_type = $2 AND entity_id = $3 ORDER BY created_at DESC",
        )
        .bind(team_id)
        .bind(entity_type)
        .bind(entity_id)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, Document>(
            "SELECT * FROM documents WHERE team_id = $1 ORDER BY created_at DESC LIMIT 50",
        )
        .bind(team_id)
        .fetch_all(&state.db)
        .await?
    };

    Ok(Json(json!({ "data": docs, "meta": null, "errors": null })))
}

async fn create_document(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateDocumentRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let doc = sqlx::query_as::<_, Document>(
        r#"INSERT INTO documents (team_id, entity_type, entity_id, file_name, file_url, file_size, mime_type, uploaded_by, description)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
           RETURNING *"#,
    )
    .bind(team_id)
    .bind(&req.entity_type)
    .bind(req.entity_id)
    .bind(&req.file_name)
    .bind(&req.file_url)
    .bind(req.file_size)
    .bind(&req.mime_type)
    .bind(req.uploaded_by)
    .bind(&req.description)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({ "data": doc, "meta": null, "errors": null })))
}

async fn get_document(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let doc = sqlx::query_as::<_, Document>(
        "SELECT * FROM documents WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Document".into()))?;

    Ok(Json(json!({ "data": doc, "meta": null, "errors": null })))
}

async fn delete_document(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    sqlx::query("DELETE FROM documents WHERE id = $1 AND team_id = $2")
        .bind(id)
        .bind(team_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({ "data": null, "meta": null, "errors": null })))
}

async fn list_signatures(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<DocumentFilter>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let sigs = if let (Some(entity_type), Some(entity_id)) = (&filter.entity_type, filter.entity_id) {
        sqlx::query_as::<_, Signature>(
            "SELECT * FROM signatures WHERE team_id = $1 AND entity_type = $2 AND entity_id = $3 ORDER BY signed_at DESC",
        )
        .bind(team_id)
        .bind(entity_type)
        .bind(entity_id)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, Signature>(
            "SELECT * FROM signatures WHERE team_id = $1 ORDER BY signed_at DESC LIMIT 50",
        )
        .bind(team_id)
        .fetch_all(&state.db)
        .await?
    };

    Ok(Json(json!({ "data": sigs, "meta": null, "errors": null })))
}

async fn create_signature(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateSignatureRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let sig = sqlx::query_as::<_, Signature>(
        r#"INSERT INTO signatures (team_id, entity_type, entity_id, signer_name, signer_email, signature_url, ip_address, signed_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
           RETURNING *"#,
    )
    .bind(team_id)
    .bind(&req.entity_type)
    .bind(req.entity_id)
    .bind(&req.signer_name)
    .bind(&req.signer_email)
    .bind(&req.signature_url)
    .bind(&req.ip_address)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({ "data": sig, "meta": null, "errors": null })))
}

#[derive(Debug, serde::Deserialize)]
struct CreateDocumentRequest {
    entity_type: String,
    entity_id: Uuid,
    file_name: String,
    file_url: String,
    file_size: i64,
    mime_type: String,
    uploaded_by: Option<Uuid>,
    description: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct CreateSignatureRequest {
    entity_type: String,
    entity_id: Uuid,
    signer_name: String,
    signer_email: Option<String>,
    signature_url: String,
    ip_address: Option<String>,
}
