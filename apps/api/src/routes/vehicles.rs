use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::models::vehicle::{CreateVehicleRequest, Vehicle, VehicleMaintenance};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/vehicles", get(list_vehicles).post(create_vehicle))
        .route("/vehicles/{id}", get(get_vehicle).patch(update_vehicle).delete(delete_vehicle))
        .route("/vehicles/{id}/maintenance", get(list_maintenance).post(create_maintenance))
}

async fn list_vehicles(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let vehicles = sqlx::query_as::<_, Vehicle>(
        "SELECT * FROM vehicles WHERE team_id = $1 ORDER BY make, model",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": vehicles,
        "meta": { "total": vehicles.len() },
        "errors": null,
    })))
}

async fn create_vehicle(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateVehicleRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let vehicle = sqlx::query_as::<_, Vehicle>(
        r#"
        INSERT INTO vehicles (team_id, make, model, year, vin, license_plate, color,
                              assigned_to, odometer, registration_expiry, insurance_policy, insurance_expiry, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(&req.make)
    .bind(&req.model)
    .bind(req.year)
    .bind(&req.vin)
    .bind(&req.license_plate)
    .bind(&req.color)
    .bind(req.assigned_to)
    .bind(req.odometer)
    .bind(req.registration_expiry)
    .bind(&req.insurance_policy)
    .bind(req.insurance_expiry)
    .bind(&req.notes)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(vehicle_id = %vehicle.id, "Vehicle created");

    Ok(Json(json!({
        "data": vehicle,
        "meta": null,
        "errors": null,
    })))
}

async fn get_vehicle(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let vehicle = sqlx::query_as::<_, Vehicle>(
        "SELECT * FROM vehicles WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Vehicle".into()))?;

    let maintenance = sqlx::query_as::<_, VehicleMaintenance>(
        "SELECT * FROM vehicle_maintenance WHERE vehicle_id = $1 ORDER BY performed_at DESC LIMIT 10",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": {
            "vehicle": vehicle,
            "maintenance": maintenance,
        },
        "meta": null,
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct UpdateVehicleRequest {
    make: Option<String>,
    model: Option<String>,
    year: Option<i32>,
    vin: Option<String>,
    license_plate: Option<String>,
    color: Option<String>,
    status: Option<String>,
    assigned_to: Option<Uuid>,
    odometer: Option<i32>,
    registration_expiry: Option<chrono::NaiveDate>,
    insurance_policy: Option<String>,
    insurance_expiry: Option<chrono::NaiveDate>,
    notes: Option<String>,
}

async fn update_vehicle(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateVehicleRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let vehicle = sqlx::query_as::<_, Vehicle>(
        r#"
        UPDATE vehicles SET
            make = COALESCE($3, make),
            model = COALESCE($4, model),
            year = COALESCE($5, year),
            vin = COALESCE($6, vin),
            license_plate = COALESCE($7, license_plate),
            color = COALESCE($8, color),
            status = COALESCE($9::vehicle_status, status),
            assigned_to = COALESCE($10, assigned_to),
            odometer = COALESCE($11, odometer),
            registration_expiry = COALESCE($12, registration_expiry),
            insurance_policy = COALESCE($13, insurance_policy),
            insurance_expiry = COALESCE($14, insurance_expiry),
            notes = COALESCE($15, notes),
            updated_at = now()
        WHERE id = $1 AND team_id = $2
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .bind(&req.make)
    .bind(&req.model)
    .bind(req.year)
    .bind(&req.vin)
    .bind(&req.license_plate)
    .bind(&req.color)
    .bind(&req.status)
    .bind(req.assigned_to)
    .bind(req.odometer)
    .bind(req.registration_expiry)
    .bind(&req.insurance_policy)
    .bind(req.insurance_expiry)
    .bind(&req.notes)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Vehicle".into()))?;

    Ok(Json(json!({
        "data": vehicle,
        "meta": null,
        "errors": null,
    })))
}

async fn delete_vehicle(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    sqlx::query("DELETE FROM vehicles WHERE id = $1 AND team_id = $2")
        .bind(id)
        .bind(team_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Vehicle deleted" },
        "errors": null,
    })))
}

async fn list_maintenance(
    State(state): State<Arc<AppState>>,
    Path(vehicle_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let records = sqlx::query_as::<_, VehicleMaintenance>(
        "SELECT * FROM vehicle_maintenance WHERE vehicle_id = $1 ORDER BY performed_at DESC",
    )
    .bind(vehicle_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": records,
        "meta": { "total": records.len() },
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct CreateMaintenanceRequest {
    maintenance_type: String,
    description: Option<String>,
    provider: Option<String>,
    cost: Option<rust_decimal::Decimal>,
    odometer: Option<i32>,
    performed_at: chrono::NaiveDate,
    next_due_date: Option<chrono::NaiveDate>,
    next_due_odometer: Option<i32>,
    notes: Option<String>,
}

async fn create_maintenance(
    State(state): State<Arc<AppState>>,
    Path(vehicle_id): Path<Uuid>,
    Json(req): Json<CreateMaintenanceRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let record = sqlx::query_as::<_, VehicleMaintenance>(
        r#"
        INSERT INTO vehicle_maintenance (vehicle_id, maintenance_type, description, provider, cost,
                                         odometer, performed_at, next_due_date, next_due_odometer, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
    )
    .bind(vehicle_id)
    .bind(&req.maintenance_type)
    .bind(&req.description)
    .bind(&req.provider)
    .bind(req.cost)
    .bind(req.odometer)
    .bind(req.performed_at)
    .bind(req.next_due_date)
    .bind(req.next_due_odometer)
    .bind(&req.notes)
    .fetch_one(&state.db)
    .await?;

    // Update vehicle odometer if provided
    if let Some(odo) = req.odometer {
        sqlx::query("UPDATE vehicles SET odometer = $2, updated_at = now() WHERE id = $1")
            .bind(vehicle_id)
            .bind(odo)
            .execute(&state.db)
            .await?;
    }

    tracing::info!(vehicle_id = %vehicle_id, maintenance_type = %req.maintenance_type, "Maintenance recorded");

    Ok(Json(json!({
        "data": record,
        "meta": null,
        "errors": null,
    })))
}
