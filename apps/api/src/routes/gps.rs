use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::Extension;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiResult;
use crate::middleware::auth::AuthUser;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/gps/location", post(update_location))
        .route("/gps/technicians", get(list_technician_locations))
        .route("/gps/technicians/{user_id}/history", get(location_history))
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct GpsLocation {
    id: Uuid,
    team_id: Uuid,
    user_id: Uuid,
    latitude: f64,
    longitude: f64,
    accuracy: Option<f64>,
    speed: Option<f64>,
    heading: Option<f64>,
    recorded_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

async fn update_location(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(req): Json<UpdateLocationRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let user_id = auth.id;

    // Upsert into a GPS tracking table (or use Redis for real-time)
    let loc = sqlx::query_as::<_, GpsLocation>(
        r#"INSERT INTO gps_locations (team_id, user_id, latitude, longitude, accuracy, speed, heading, recorded_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, COALESCE($8, NOW()))
           RETURNING *"#,
    )
    .bind(team_id)
    .bind(user_id)
    .bind(req.latitude)
    .bind(req.longitude)
    .bind(req.accuracy)
    .bind(req.speed)
    .bind(req.heading)
    .bind(req.recorded_at)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({ "data": loc, "meta": null, "errors": null })))
}

async fn list_technician_locations(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    // Get the latest location for each technician
    let locations = sqlx::query_as::<_, GpsLocation>(
        r#"SELECT DISTINCT ON (user_id) *
           FROM gps_locations
           WHERE team_id = $1 AND recorded_at > NOW() - INTERVAL '1 hour'
           ORDER BY user_id, recorded_at DESC"#,
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({ "data": locations, "meta": null, "errors": null })))
}

async fn location_history(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let history = sqlx::query_as::<_, GpsLocation>(
        r#"SELECT * FROM gps_locations
           WHERE team_id = $1 AND user_id = $2 AND recorded_at > NOW() - INTERVAL '24 hours'
           ORDER BY recorded_at DESC
           LIMIT 500"#,
    )
    .bind(team_id)
    .bind(user_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({ "data": history, "meta": null, "errors": null })))
}

#[derive(Debug, Deserialize)]
struct UpdateLocationRequest {
    latitude: f64,
    longitude: f64,
    accuracy: Option<f64>,
    speed: Option<f64>,
    heading: Option<f64>,
    recorded_at: Option<DateTime<Utc>>,
}
