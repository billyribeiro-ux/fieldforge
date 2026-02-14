use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use axum::Extension;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::middleware::auth::AuthUser;
use crate::models::document::FuelLog;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/vehicles/{vehicle_id}/fuel-logs", get(list_fuel_logs).post(create_fuel_log))
        .route("/fuel-logs/{id}", get(get_fuel_log).delete(delete_fuel_log))
}

async fn list_fuel_logs(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(vehicle_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let logs = sqlx::query_as::<_, FuelLog>(
        "SELECT * FROM fuel_logs WHERE vehicle_id = $1 AND team_id = $2 ORDER BY filled_at DESC",
    )
    .bind(vehicle_id)
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({ "data": logs, "meta": null, "errors": null })))
}

async fn create_fuel_log(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(vehicle_id): Path<Uuid>,
    Json(req): Json<CreateFuelLogRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let log = sqlx::query_as::<_, FuelLog>(
        r#"INSERT INTO fuel_logs (vehicle_id, team_id, gallons, cost_per_gallon, total_cost, odometer, fuel_type, station, filled_by, filled_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, COALESCE($10, NOW()))
           RETURNING *"#,
    )
    .bind(vehicle_id)
    .bind(team_id)
    .bind(req.gallons)
    .bind(req.cost_per_gallon)
    .bind(req.gallons * req.cost_per_gallon)
    .bind(req.odometer)
    .bind(&req.fuel_type)
    .bind(&req.station)
    .bind(req.filled_by)
    .bind(req.filled_at)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({ "data": log, "meta": null, "errors": null })))
}

async fn get_fuel_log(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let log = sqlx::query_as::<_, FuelLog>(
        "SELECT * FROM fuel_logs WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Fuel log".into()))?;

    Ok(Json(json!({ "data": log, "meta": null, "errors": null })))
}

async fn delete_fuel_log(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    sqlx::query("DELETE FROM fuel_logs WHERE id = $1 AND team_id = $2")
        .bind(id)
        .bind(team_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({ "data": null, "meta": null, "errors": null })))
}

#[derive(Debug, serde::Deserialize)]
struct CreateFuelLogRequest {
    gallons: rust_decimal::Decimal,
    cost_per_gallon: rust_decimal::Decimal,
    odometer: Option<i32>,
    fuel_type: Option<String>,
    station: Option<String>,
    filled_by: Option<Uuid>,
    filled_at: Option<chrono::DateTime<chrono::Utc>>,
}
