use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use axum::Extension;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::middleware::auth::AuthUser;
use crate::models::document::PurchaseOrder;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/purchase-orders", get(list_orders).post(create_order))
        .route(
            "/purchase-orders/{id}",
            get(get_order).patch(update_order).delete(delete_order),
        )
        .route("/purchase-orders/{id}/receive", axum::routing::post(receive_order))
}

async fn list_orders(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let orders = sqlx::query_as::<_, PurchaseOrder>(
        "SELECT * FROM purchase_orders WHERE team_id = $1 ORDER BY created_at DESC",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({ "data": orders, "meta": null, "errors": null })))
}

async fn create_order(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(req): Json<CreatePurchaseOrderRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let order = sqlx::query_as::<_, PurchaseOrder>(
        r#"INSERT INTO purchase_orders (team_id, job_id, vendor, po_number, status, subtotal, tax, total, notes, expected_date, created_by)
           VALUES ($1, $2, $3, $4, 'draft'::po_status, $5, $6, $7, $8, $9, $10)
           RETURNING *"#,
    )
    .bind(team_id)
    .bind(req.job_id)
    .bind(&req.vendor)
    .bind(&req.po_number)
    .bind(req.subtotal)
    .bind(req.tax)
    .bind(req.subtotal + req.tax)
    .bind(&req.notes)
    .bind(req.expected_date)
    .bind(req.created_by)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({ "data": order, "meta": null, "errors": null })))
}

async fn get_order(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let order = sqlx::query_as::<_, PurchaseOrder>(
        "SELECT * FROM purchase_orders WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Purchase order".into()))?;

    Ok(Json(json!({ "data": order, "meta": null, "errors": null })))
}

async fn update_order(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let order = sqlx::query_as::<_, PurchaseOrder>(
        r#"UPDATE purchase_orders
           SET vendor = COALESCE($3, vendor),
               notes = COALESCE($4, notes),
               expected_date = COALESCE($5, expected_date),
               updated_at = NOW()
           WHERE id = $1 AND team_id = $2
           RETURNING *"#,
    )
    .bind(id)
    .bind(team_id)
    .bind(req.get("vendor").and_then(|v| v.as_str()))
    .bind(req.get("notes").and_then(|v| v.as_str()))
    .bind(req.get("expected_date").and_then(|v| v.as_str()))
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({ "data": order, "meta": null, "errors": null })))
}

async fn delete_order(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    sqlx::query("DELETE FROM purchase_orders WHERE id = $1 AND team_id = $2")
        .bind(id)
        .bind(team_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({ "data": null, "meta": null, "errors": null })))
}

async fn receive_order(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let order = sqlx::query_as::<_, PurchaseOrder>(
        r#"UPDATE purchase_orders
           SET status = 'received'::po_status, received_date = CURRENT_DATE, updated_at = NOW()
           WHERE id = $1 AND team_id = $2
           RETURNING *"#,
    )
    .bind(id)
    .bind(team_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({ "data": order, "meta": null, "errors": null })))
}

#[derive(Debug, serde::Deserialize)]
struct CreatePurchaseOrderRequest {
    job_id: Option<Uuid>,
    vendor: String,
    po_number: String,
    subtotal: rust_decimal::Decimal,
    tax: rust_decimal::Decimal,
    notes: Option<String>,
    expected_date: Option<chrono::NaiveDate>,
    created_by: Option<Uuid>,
}
