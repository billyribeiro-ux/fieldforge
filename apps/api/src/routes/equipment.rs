use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::models::equipment::{CreateEquipmentRequest, Equipment};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/equipment", get(list_equipment).post(create_equipment))
        .route("/equipment/{id}", get(get_equipment).patch(update_equipment).delete(delete_equipment))
}

async fn list_equipment(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let items = sqlx::query_as::<_, Equipment>(
        "SELECT * FROM equipment WHERE team_id = $1 AND is_active = true ORDER BY name",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": items,
        "meta": { "total": items.len() },
        "errors": null,
    })))
}

async fn create_equipment(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateEquipmentRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();
    let condition = req.condition.as_deref().unwrap_or("good");

    let item = sqlx::query_as::<_, Equipment>(
        r#"
        INSERT INTO equipment (team_id, name, category, brand, model, serial_number,
                               purchase_date, purchase_price, warranty_expiry, assigned_to,
                               condition, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11::equipment_condition, $12)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(&req.name)
    .bind(&req.category)
    .bind(&req.brand)
    .bind(&req.model)
    .bind(&req.serial_number)
    .bind(req.purchase_date)
    .bind(req.purchase_price)
    .bind(req.warranty_expiry)
    .bind(req.assigned_to)
    .bind(condition)
    .bind(&req.notes)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(equipment_id = %item.id, name = %req.name, "Equipment created");

    Ok(Json(json!({
        "data": item,
        "meta": null,
        "errors": null,
    })))
}

async fn get_equipment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let item = sqlx::query_as::<_, Equipment>(
        "SELECT * FROM equipment WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Equipment".into()))?;

    Ok(Json(json!({
        "data": item,
        "meta": null,
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct UpdateEquipmentRequest {
    name: Option<String>,
    category: Option<String>,
    brand: Option<String>,
    model: Option<String>,
    serial_number: Option<String>,
    condition: Option<String>,
    assigned_to: Option<Uuid>,
    notes: Option<String>,
}

async fn update_equipment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateEquipmentRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let item = sqlx::query_as::<_, Equipment>(
        r#"
        UPDATE equipment SET
            name = COALESCE($3, name),
            category = COALESCE($4, category),
            brand = COALESCE($5, brand),
            model = COALESCE($6, model),
            serial_number = COALESCE($7, serial_number),
            condition = COALESCE($8::equipment_condition, condition),
            assigned_to = COALESCE($9, assigned_to),
            notes = COALESCE($10, notes),
            updated_at = now()
        WHERE id = $1 AND team_id = $2
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .bind(&req.name)
    .bind(&req.category)
    .bind(&req.brand)
    .bind(&req.model)
    .bind(&req.serial_number)
    .bind(&req.condition)
    .bind(req.assigned_to)
    .bind(&req.notes)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Equipment".into()))?;

    Ok(Json(json!({
        "data": item,
        "meta": null,
        "errors": null,
    })))
}

async fn delete_equipment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    sqlx::query(
        "UPDATE equipment SET is_active = false, updated_at = now() WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .execute(&state.db)
    .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Equipment deactivated" },
        "errors": null,
    })))
}
