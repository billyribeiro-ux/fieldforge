use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use axum::Extension;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::middleware::auth::AuthUser;
use crate::models::review::{CreateReviewRequest, Review};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/reviews", get(list_reviews).post(create_review))
        .route("/reviews/{id}", get(get_review).patch(update_review))
        .route("/reviews/{id}/respond", axum::routing::post(respond_to_review))
}

async fn list_reviews(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let reviews = sqlx::query_as::<_, Review>(
        "SELECT * FROM reviews WHERE team_id = $1 ORDER BY created_at DESC",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    let avg_rating: f64 = if reviews.is_empty() {
        0.0
    } else {
        reviews.iter().map(|r| r.rating as f64).sum::<f64>() / reviews.len() as f64
    };

    Ok(Json(json!({
        "data": reviews,
        "meta": {
            "total": reviews.len(),
            "average_rating": (avg_rating * 10.0).round() / 10.0,
        },
        "errors": null,
    })))
}

async fn create_review(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(req): Json<CreateReviewRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let review = sqlx::query_as::<_, Review>(
        r#"
        INSERT INTO reviews (team_id, customer_id, job_id, platform, rating, content, reviewer_name)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(req.customer_id)
    .bind(req.job_id)
    .bind(&req.platform)
    .bind(req.rating)
    .bind(&req.content)
    .bind(&req.reviewer_name)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(review_id = %review.id, rating = review.rating, "Review created");

    Ok(Json(json!({
        "data": review,
        "meta": null,
        "errors": null,
    })))
}

async fn get_review(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let review = sqlx::query_as::<_, Review>(
        "SELECT * FROM reviews WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Review".into()))?;

    Ok(Json(json!({
        "data": review,
        "meta": null,
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct UpdateReviewRequest {
    rating: Option<i16>,
    content: Option<String>,
}

async fn update_review(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateReviewRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let review = sqlx::query_as::<_, Review>(
        r#"
        UPDATE reviews SET
            rating = COALESCE($2, rating),
            content = COALESCE($3, content),
            updated_at = now()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(req.rating)
    .bind(&req.content)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Review".into()))?;

    Ok(Json(json!({
        "data": review,
        "meta": null,
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct RespondRequest {
    response: String,
}

async fn respond_to_review(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<RespondRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let review = sqlx::query_as::<_, Review>(
        r#"
        UPDATE reviews SET
            response = $2,
            responded_at = now(),
            updated_at = now()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(&req.response)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Review".into()))?;

    tracing::info!(review_id = %id, "Review response added");

    Ok(Json(json!({
        "data": review,
        "meta": null,
        "errors": null,
    })))
}
