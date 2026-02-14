use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum::Extension;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::middleware::auth::AuthUser;
use crate::models::common::PaginationParams;
use crate::models::estimate::CreateEstimateRequest;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/estimates", get(list_estimates).post(create_estimate))
        .route("/estimates/{id}", get(get_estimate).patch(update_estimate))
        .route("/estimates/{id}/send", post(send_estimate))
        .route("/estimates/{id}/approve", post(approve_estimate))
        .route("/estimates/{id}/decline", post(decline_estimate))
        .route("/estimates/{id}/convert", post(convert_to_invoice))
        .route("/estimates/{id}/duplicate", post(duplicate_estimate))
}

async fn create_estimate(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(req): Json<CreateEstimateRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let mut tx = state.db.begin().await?;

    // Generate estimate number
    let estimate_number = sqlx::query_scalar::<_, String>(
        r#"
        UPDATE teams SET estimate_next_number = estimate_next_number + 1
        WHERE id = $1
        RETURNING estimate_prefix || '-' || LPAD((estimate_next_number - 1)::text, 4, '0')
        "#,
    )
    .bind(team_id)
    .fetch_one(&mut *tx)
    .await?;

    // Create estimate
    let estimate_id = Uuid::new_v4();
    let tax_rate: rust_decimal::Decimal = sqlx::query_scalar(
        "SELECT COALESCE(tax_rate, 0) FROM teams WHERE id = $1",
    )
    .bind(team_id)
    .fetch_one(&mut *tx)
    .await
    .unwrap_or_default();

    // Calculate totals from line items
    let mut subtotal = rust_decimal::Decimal::ZERO;
    let mut taxable_subtotal = rust_decimal::Decimal::ZERO;

    for item in &req.line_items {
        let line_total = item.quantity * item.unit_price;
        subtotal += line_total;
        if item.taxable.unwrap_or(true) {
            taxable_subtotal += line_total;
        }
    }

    let discount_amount = req.discount_amount.unwrap_or_default();
    let tax_amount = (taxable_subtotal - discount_amount).max(rust_decimal::Decimal::ZERO) * tax_rate / rust_decimal::Decimal::from(100);
    let total = subtotal - discount_amount + tax_amount;

    let estimate = sqlx::query_as::<_, crate::models::estimate::Estimate>(
        r#"
        INSERT INTO estimates (id, team_id, customer_id, job_id, property_id, estimate_number, title, scope_of_work,
                               subtotal, discount_amount, discount_pct, tax_amount, tax_rate, total,
                               valid_until, payment_terms, warranty_terms)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        RETURNING *
        "#,
    )
    .bind(estimate_id)
    .bind(team_id)
    .bind(req.customer_id)
    .bind(req.job_id)
    .bind(req.property_id)
    .bind(&estimate_number)
    .bind(&req.title)
    .bind(&req.scope_of_work)
    .bind(subtotal)
    .bind(discount_amount)
    .bind(req.discount_pct)
    .bind(tax_amount)
    .bind(tax_rate)
    .bind(total)
    .bind(req.valid_until)
    .bind(&req.payment_terms)
    .bind(&req.warranty_terms)
    .fetch_one(&mut *tx)
    .await?;

    // Insert line items
    for (i, item) in req.line_items.iter().enumerate() {
        let line_total = item.quantity * item.unit_price;
        sqlx::query(
            r#"
            INSERT INTO line_items (team_id, estimate_id, description, category, quantity, unit, unit_price, total, taxable, sort_order)
            VALUES ($1, $2, $3, COALESCE($4, 'other')::line_item_category, $5, COALESCE($6, 'each'), $7, $8, $9, $10)
            "#,
        )
        .bind(team_id)
        .bind(estimate_id)
        .bind(&item.description)
        .bind(&item.category)
        .bind(item.quantity)
        .bind(&item.unit)
        .bind(item.unit_price)
        .bind(line_total)
        .bind(item.taxable.unwrap_or(true))
        .bind(item.sort_order.unwrap_or(i as i32))
        .execute(&mut *tx)
        .await?;
    }

    // Fetch line items for response
    let line_items = sqlx::query_as::<_, crate::models::line_item::LineItem>(
        "SELECT * FROM line_items WHERE estimate_id = $1 ORDER BY sort_order",
    )
    .bind(estimate_id)
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(json!({
        "data": {
            "estimate": estimate,
            "line_items": line_items,
        },
        "meta": null,
        "errors": null,
    })))
}

async fn list_estimates(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Query(pagination): Query<PaginationParams>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let status = params.get("status").map(|s| s.as_str());
    let cursor = pagination.cursor.as_ref().and_then(|c| c.parse::<Uuid>().ok());

    let estimates = if let Some(status_filter) = status {
        sqlx::query_as::<_, crate::models::estimate::Estimate>(
            r#"
            SELECT * FROM estimates
            WHERE team_id = $1 AND deleted_at IS NULL AND status::text = $2
              AND ($3::uuid IS NULL OR id < $3)
            ORDER BY created_at DESC LIMIT $4
            "#,
        )
        .bind(team_id)
        .bind(status_filter)
        .bind(cursor)
        .bind(pagination.limit())
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, crate::models::estimate::Estimate>(
            r#"
            SELECT * FROM estimates
            WHERE team_id = $1 AND deleted_at IS NULL
              AND ($2::uuid IS NULL OR id < $2)
            ORDER BY created_at DESC LIMIT $3
            "#,
        )
        .bind(team_id)
        .bind(cursor)
        .bind(pagination.limit())
        .fetch_all(&state.db)
        .await?
    };

    let has_more = estimates.len() as i64 == pagination.limit();

    Ok(Json(json!({
        "data": estimates,
        "meta": { "has_more": has_more, "cursor": estimates.last().map(|e| e.id.to_string()) },
        "errors": null,
    })))
}

async fn get_estimate(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let estimate = sqlx::query_as::<_, crate::models::estimate::Estimate>(
        "SELECT * FROM estimates WHERE id = $1 AND team_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Estimate".into()))?;

    let line_items = sqlx::query_as::<_, crate::models::line_item::LineItem>(
        "SELECT * FROM line_items WHERE estimate_id = $1 ORDER BY sort_order",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": {
            "estimate": estimate,
            "line_items": line_items,
        },
        "meta": null,
        "errors": null,
    })))
}

async fn update_estimate(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<Uuid>,
    Json(_body): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    // TODO: implement partial update for estimate fields + line items
    Ok(Json(json!({ "data": null, "meta": { "message": "Not yet implemented" }, "errors": null })))
}

async fn send_estimate(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let estimate = sqlx::query_as::<_, crate::models::estimate::Estimate>(
        r#"
        UPDATE estimates SET status = 'sent'::estimate_status, sent_at = now()
        WHERE id = $1 AND team_id = $2 AND status = 'draft'::estimate_status AND deleted_at IS NULL
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::BadRequest("Estimate must be in draft status to send".into()))?;

    // TODO: Send email/SMS to customer with portal link
    tracing::info!(estimate_id = %id, "Estimate sent to customer");

    Ok(Json(json!({
        "data": estimate,
        "meta": { "message": "Estimate sent successfully" },
        "errors": null,
    })))
}

#[derive(serde::Deserialize)]
struct ApproveRequest {
    signature: Option<String>,
}

async fn approve_estimate(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<ApproveRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let estimate = sqlx::query_as::<_, crate::models::estimate::Estimate>(
        r#"
        UPDATE estimates SET
            status = 'approved'::estimate_status,
            approved_at = now(),
            customer_signature = $3
        WHERE id = $1 AND team_id = $2 AND status IN ('sent'::estimate_status, 'viewed'::estimate_status) AND deleted_at IS NULL
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .bind(&req.signature)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::BadRequest("Estimate must be sent or viewed to approve".into()))?;

    // Auto-update job status if linked (only if transition is valid)
    if let Some(job_id) = estimate.job_id {
        let job = sqlx::query_as::<_, crate::models::job::Job>(
            "SELECT * FROM jobs WHERE id = $1 AND team_id = $2",
        )
        .bind(job_id)
        .bind(team_id)
        .fetch_optional(&state.db)
        .await?;

        if let Some(job) = job {
            if crate::services::job_service::is_valid_transition(&job.status, &"approved".to_string()) {
                sqlx::query("UPDATE jobs SET status = 'approved'::job_status WHERE id = $1")
                    .bind(job_id)
                    .execute(&state.db)
                    .await?;
            }
        }
    }

    Ok(Json(json!({
        "data": estimate,
        "meta": { "message": "Estimate approved" },
        "errors": null,
    })))
}

#[derive(serde::Deserialize)]
struct DeclineRequest {
    reason: Option<String>,
}

async fn decline_estimate(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<DeclineRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let estimate = sqlx::query_as::<_, crate::models::estimate::Estimate>(
        r#"
        UPDATE estimates SET
            status = 'declined'::estimate_status,
            declined_at = now(),
            decline_reason = $3
        WHERE id = $1 AND team_id = $2 AND status IN ('sent'::estimate_status, 'viewed'::estimate_status) AND deleted_at IS NULL
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .bind(&req.reason)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::BadRequest("Estimate must be sent or viewed to decline".into()))?;

    Ok(Json(json!({
        "data": estimate,
        "meta": { "message": "Estimate declined" },
        "errors": null,
    })))
}

async fn convert_to_invoice(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let mut tx = state.db.begin().await?;

    // Get approved estimate
    let estimate = sqlx::query_as::<_, crate::models::estimate::Estimate>(
        "SELECT * FROM estimates WHERE id = $1 AND team_id = $2 AND status = 'approved'::estimate_status AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| ApiError::BadRequest("Only approved estimates can be converted to invoices".into()))?;

    // Generate invoice number
    let invoice_number = sqlx::query_scalar::<_, String>(
        r#"
        UPDATE teams SET invoice_next_number = invoice_next_number + 1
        WHERE id = $1
        RETURNING invoice_prefix || '-' || LPAD((invoice_next_number - 1)::text, 4, '0')
        "#,
    )
    .bind(team_id)
    .fetch_one(&mut *tx)
    .await?;

    // Create invoice from estimate
    let invoice_id = Uuid::new_v4();
    let invoice = sqlx::query_as::<_, crate::models::invoice::Invoice>(
        r#"
        INSERT INTO invoices (id, team_id, job_id, estimate_id, customer_id, property_id, invoice_number,
                              subtotal, discount_amount, tax_amount, tax_rate, total, amount_due,
                              due_date, payment_terms)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $12, $13, $14)
        RETURNING *
        "#,
    )
    .bind(invoice_id)
    .bind(team_id)
    .bind(estimate.job_id)
    .bind(id)
    .bind(estimate.customer_id)
    .bind(estimate.property_id)
    .bind(&invoice_number)
    .bind(estimate.subtotal)
    .bind(estimate.discount_amount)
    .bind(estimate.tax_amount)
    .bind(estimate.tax_rate)
    .bind(estimate.total)
    .bind(chrono::Utc::now().date_naive() + chrono::Duration::days(30))
    .bind(&estimate.payment_terms)
    .fetch_one(&mut *tx)
    .await?;

    // Copy line items from estimate to invoice
    sqlx::query(
        r#"
        INSERT INTO line_items (team_id, invoice_id, description, category, quantity, unit, unit_price, total, cost_price, taxable, sort_order)
        SELECT team_id, $2, description, category, quantity, unit, unit_price, total, cost_price, taxable, sort_order
        FROM line_items WHERE estimate_id = $1
        "#,
    )
    .bind(id)
    .bind(invoice_id)
    .execute(&mut *tx)
    .await?;

    // Update estimate status
    sqlx::query("UPDATE estimates SET status = 'converted'::estimate_status WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(Json(json!({
        "data": invoice,
        "meta": { "message": "Invoice created from estimate" },
        "errors": null,
    })))
}

async fn duplicate_estimate(
    State(_state): State<Arc<AppState>>,
    Path(_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    // TODO: duplicate estimate with new number + draft status
    Ok(Json(json!({ "data": null, "meta": { "message": "Not yet implemented" }, "errors": null })))
}
