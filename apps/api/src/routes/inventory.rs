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
use crate::models::inventory::{CreateInventoryItemRequest, InventoryItem, InventoryLocation, InventoryStock};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/inventory/items", get(list_items).post(create_item))
        .route("/inventory/items/{id}", get(get_item).patch(update_item).delete(delete_item))
        .route("/inventory/locations", get(list_locations).post(create_location))
        .route("/inventory/items/{id}/stock", get(get_item_stock))
        .route("/inventory/items/{id}/adjust", axum::routing::post(adjust_stock))
}

async fn list_items(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let items = sqlx::query_as::<_, InventoryItem>(
        "SELECT * FROM inventory_items WHERE team_id = $1 AND is_active = true ORDER BY name",
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

async fn create_item(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(req): Json<CreateInventoryItemRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let unit = req.unit_of_measure.as_deref().unwrap_or("each");

    let item = sqlx::query_as::<_, InventoryItem>(
        r#"
        INSERT INTO inventory_items (team_id, name, sku, description, category, unit_of_measure,
                                     min_stock_level, cost_price, sell_price, preferred_supplier)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(&req.name)
    .bind(&req.sku)
    .bind(&req.description)
    .bind(&req.category)
    .bind(unit)
    .bind(req.min_stock_level)
    .bind(req.cost_price)
    .bind(req.sell_price)
    .bind(&req.preferred_supplier)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(item_id = %item.id, name = %req.name, "Inventory item created");

    Ok(Json(json!({
        "data": item,
        "meta": null,
        "errors": null,
    })))
}

async fn get_item(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let item = sqlx::query_as::<_, InventoryItem>(
        "SELECT * FROM inventory_items WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Inventory item".into()))?;

    let stock = sqlx::query_as::<_, InventoryStock>(
        "SELECT * FROM inventory_stock WHERE item_id = $1",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": {
            "item": item,
            "stock": stock,
        },
        "meta": null,
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct UpdateInventoryItemRequest {
    name: Option<String>,
    sku: Option<String>,
    description: Option<String>,
    category: Option<String>,
    min_stock_level: Option<rust_decimal::Decimal>,
    cost_price: Option<rust_decimal::Decimal>,
    sell_price: Option<rust_decimal::Decimal>,
    preferred_supplier: Option<String>,
}

async fn update_item(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateInventoryItemRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let item = sqlx::query_as::<_, InventoryItem>(
        r#"
        UPDATE inventory_items SET
            name = COALESCE($3, name),
            sku = COALESCE($4, sku),
            description = COALESCE($5, description),
            category = COALESCE($6, category),
            min_stock_level = COALESCE($7, min_stock_level),
            cost_price = COALESCE($8, cost_price),
            sell_price = COALESCE($9, sell_price),
            preferred_supplier = COALESCE($10, preferred_supplier),
            updated_at = now()
        WHERE id = $1 AND team_id = $2
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .bind(&req.name)
    .bind(&req.sku)
    .bind(&req.description)
    .bind(&req.category)
    .bind(req.min_stock_level)
    .bind(req.cost_price)
    .bind(req.sell_price)
    .bind(&req.preferred_supplier)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Inventory item".into()))?;

    Ok(Json(json!({
        "data": item,
        "meta": null,
        "errors": null,
    })))
}

async fn delete_item(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    sqlx::query(
        "UPDATE inventory_items SET is_active = false, updated_at = now() WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .execute(&state.db)
    .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Item deactivated" },
        "errors": null,
    })))
}

async fn list_locations(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let locations = sqlx::query_as::<_, InventoryLocation>(
        "SELECT * FROM inventory_locations WHERE team_id = $1 AND is_active = true ORDER BY name",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": locations,
        "meta": { "total": locations.len() },
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct CreateLocationRequest {
    name: String,
    location_type: Option<String>,
    vehicle_id: Option<Uuid>,
    address: Option<String>,
}

async fn create_location(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(req): Json<CreateLocationRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let loc_type = req.location_type.as_deref().unwrap_or("warehouse");

    let location = sqlx::query_as::<_, InventoryLocation>(
        r#"
        INSERT INTO inventory_locations (team_id, name, location_type, vehicle_id, address)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(&req.name)
    .bind(loc_type)
    .bind(req.vehicle_id)
    .bind(&req.address)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({
        "data": location,
        "meta": null,
        "errors": null,
    })))
}

async fn get_item_stock(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let stock = sqlx::query_as::<_, InventoryStock>(
        "SELECT * FROM inventory_stock WHERE item_id = $1",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    let total_qty: rust_decimal::Decimal = stock.iter().map(|s| s.quantity).sum();

    Ok(Json(json!({
        "data": stock,
        "meta": { "total_quantity": total_qty },
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct AdjustStockRequest {
    location_id: Uuid,
    quantity: rust_decimal::Decimal,
    txn_type: String,
    job_id: Option<Uuid>,
    notes: Option<String>,
}

async fn adjust_stock(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<AdjustStockRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let user_id = auth.id;

    let mut tx = state.db.begin().await?;

    // Upsert stock level
    sqlx::query(
        r#"
        INSERT INTO inventory_stock (item_id, location_id, quantity)
        VALUES ($1, $2, $3)
        ON CONFLICT (item_id, location_id)
        DO UPDATE SET quantity = inventory_stock.quantity + $3, updated_at = now()
        "#,
    )
    .bind(id)
    .bind(req.location_id)
    .bind(req.quantity)
    .execute(&mut *tx)
    .await?;

    // Record transaction
    sqlx::query(
        r#"
        INSERT INTO inventory_transactions (team_id, item_id, location_id, txn_type, quantity, job_id, notes, performed_by)
        VALUES ($1, $2, $3, $4::inventory_txn_type, $5, $6, $7, $8)
        "#,
    )
    .bind(team_id)
    .bind(id)
    .bind(req.location_id)
    .bind(&req.txn_type)
    .bind(req.quantity)
    .bind(req.job_id)
    .bind(&req.notes)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    tracing::info!(item_id = %id, qty = %req.quantity, txn_type = %req.txn_type, "Stock adjusted");

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Stock adjusted" },
        "errors": null,
    })))
}
