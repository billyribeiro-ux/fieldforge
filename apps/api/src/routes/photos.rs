use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/jobs/{job_id}/photos", get(list_job_photos).post(create_photo))
        .route("/photos/{id}", get(get_photo).delete(delete_photo))
        .route("/photos/presigned-url", post(get_presigned_upload_url))
}

#[derive(Deserialize)]
struct PresignedUrlRequest {
    filename: String,
    content_type: String,
    job_id: Option<Uuid>,
    category: Option<String>,
}

#[derive(Serialize)]
struct PresignedUrlResponse {
    upload_url: String,
    file_key: String,
    expires_in: u64,
}

async fn get_presigned_upload_url(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<PresignedUrlRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();
    let file_id = Uuid::new_v4();

    // Generate S3 key: team_id/photos/year/month/file_id.ext
    let now = chrono::Utc::now();
    let ext = req.filename.rsplit('.').next().unwrap_or("jpg");
    let file_key = format!(
        "{}/photos/{}/{}/{}.{}",
        team_id,
        now.format("%Y"),
        now.format("%m"),
        file_id,
        ext
    );

    // Generate presigned PUT URL using AWS SDK
    // In production, this would use aws_sdk_s3::presigning
    let upload_url = format!(
        "{}/{}/{}",
        std::env::var("S3_ENDPOINT").unwrap_or_else(|_| "http://localhost:9000".to_string()),
        std::env::var("S3_BUCKET").unwrap_or_else(|_| "fieldforge-uploads".to_string()),
        file_key
    );

    // TODO: Replace with actual presigned URL generation:
    // let presigned = s3_client
    //     .put_object()
    //     .bucket(&bucket)
    //     .key(&file_key)
    //     .content_type(&req.content_type)
    //     .presigned(PresigningConfig::expires_in(Duration::from_secs(3600))?)
    //     .await?;

    Ok(Json(json!({
        "data": {
            "upload_url": upload_url,
            "file_key": file_key,
            "expires_in": 3600,
        },
        "meta": null,
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct CreatePhotoRequest {
    job_id: Uuid,
    file_key: String,
    filename: String,
    content_type: String,
    file_size: Option<i64>,
    category: Option<String>,
    caption: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

async fn create_photo(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<Uuid>,
    Json(req): Json<CreatePhotoRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();
    let user_id = Uuid::nil();

    let category = req.category.as_deref().unwrap_or("general");

    let photo = sqlx::query_as::<_, crate::models::photo::Photo>(
        r#"
        INSERT INTO photos (team_id, job_id, uploaded_by, file_key, filename, content_type, file_size,
                            category, caption, latitude, longitude)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8::photo_category, $9, $10, $11)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(job_id)
    .bind(user_id)
    .bind(&req.file_key)
    .bind(&req.filename)
    .bind(&req.content_type)
    .bind(req.file_size)
    .bind(category)
    .bind(&req.caption)
    .bind(req.latitude)
    .bind(req.longitude)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(photo_id = %photo.id, job_id = %job_id, "Photo uploaded");

    Ok(Json(json!({
        "data": photo,
        "meta": null,
        "errors": null,
    })))
}

async fn list_job_photos(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let photos = sqlx::query_as::<_, crate::models::photo::Photo>(
        "SELECT * FROM photos WHERE job_id = $1 AND deleted_at IS NULL ORDER BY created_at DESC",
    )
    .bind(job_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": photos,
        "meta": { "total": photos.len() },
        "errors": null,
    })))
}

async fn get_photo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let photo = sqlx::query_as::<_, crate::models::photo::Photo>(
        "SELECT * FROM photos WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Photo".into()))?;

    // Generate presigned GET URL for viewing
    let view_url = format!(
        "{}/{}/{}",
        std::env::var("S3_ENDPOINT").unwrap_or_else(|_| "http://localhost:9000".to_string()),
        std::env::var("S3_BUCKET").unwrap_or_else(|_| "fieldforge-uploads".to_string()),
        photo.file_key
    );

    Ok(Json(json!({
        "data": {
            "photo": photo,
            "view_url": view_url,
        },
        "meta": null,
        "errors": null,
    })))
}

async fn delete_photo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    sqlx::query(
        "UPDATE photos SET deleted_at = now() WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(&state.db)
    .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Photo deleted" },
        "errors": null,
    })))
}
