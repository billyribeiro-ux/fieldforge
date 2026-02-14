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
use crate::models::invoice::CreateInvoiceRequest;
use crate::models::payment::RecordPaymentRequest;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/invoices", get(list_invoices).post(create_invoice))
        .route("/invoices/{id}", get(get_invoice))
        .route("/invoices/{id}/send", post(send_invoice))
        .route("/invoices/{id}/void", post(void_invoice))
        .route("/invoices/{id}/payments", get(list_payments).post(record_payment))
}

async fn create_invoice(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(req): Json<CreateInvoiceRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let mut tx = state.db.begin().await?;

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

    let invoice_id = Uuid::new_v4();
    let tax_rate: rust_decimal::Decimal = sqlx::query_scalar(
        "SELECT COALESCE(tax_rate, 0) FROM teams WHERE id = $1",
    )
    .bind(team_id)
    .fetch_one(&mut *tx)
    .await
    .unwrap_or_default();

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

    let due_date = req.due_date.unwrap_or_else(|| {
        (chrono::Utc::now() + chrono::Duration::days(30)).date_naive()
    });

    let invoice = sqlx::query_as::<_, crate::models::invoice::Invoice>(
        r#"
        INSERT INTO invoices (id, team_id, job_id, estimate_id, customer_id, property_id, invoice_number,
                              subtotal, discount_amount, tax_amount, tax_rate, total, amount_due, due_date, payment_terms, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $12, $13, $14, $15)
        RETURNING *
        "#,
    )
    .bind(invoice_id)
    .bind(team_id)
    .bind(req.job_id)
    .bind(req.estimate_id)
    .bind(req.customer_id)
    .bind(req.property_id)
    .bind(&invoice_number)
    .bind(subtotal)
    .bind(discount_amount)
    .bind(tax_amount)
    .bind(tax_rate)
    .bind(total)
    .bind(due_date)
    .bind(&req.payment_terms)
    .bind(&req.notes)
    .fetch_one(&mut *tx)
    .await?;

    for (i, item) in req.line_items.iter().enumerate() {
        let line_total = item.quantity * item.unit_price;
        sqlx::query(
            r#"
            INSERT INTO line_items (team_id, invoice_id, description, category, quantity, unit, unit_price, total, taxable, sort_order)
            VALUES ($1, $2, $3, COALESCE($4, 'other')::line_item_category, $5, COALESCE($6, 'each'), $7, $8, $9, $10)
            "#,
        )
        .bind(team_id)
        .bind(invoice_id)
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

    let line_items = sqlx::query_as::<_, crate::models::line_item::LineItem>(
        "SELECT * FROM line_items WHERE invoice_id = $1 ORDER BY sort_order",
    )
    .bind(invoice_id)
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(json!({
        "data": { "invoice": invoice, "line_items": line_items },
        "meta": null,
        "errors": null,
    })))
}

async fn list_invoices(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Query(pagination): Query<PaginationParams>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let status = params.get("status").map(|s| s.as_str());
    let cursor = pagination.cursor.as_ref().and_then(|c| c.parse::<Uuid>().ok());

    let invoices = if let Some(status_filter) = status {
        sqlx::query_as::<_, crate::models::invoice::Invoice>(
            r#"
            SELECT * FROM invoices
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
        sqlx::query_as::<_, crate::models::invoice::Invoice>(
            r#"
            SELECT * FROM invoices
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

    let has_more = invoices.len() as i64 == pagination.limit();

    Ok(Json(json!({
        "data": invoices,
        "meta": { "has_more": has_more, "cursor": invoices.last().map(|i| i.id.to_string()) },
        "errors": null,
    })))
}

async fn get_invoice(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let invoice = sqlx::query_as::<_, crate::models::invoice::Invoice>(
        "SELECT * FROM invoices WHERE id = $1 AND team_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Invoice".into()))?;

    let line_items = sqlx::query_as::<_, crate::models::line_item::LineItem>(
        "SELECT * FROM line_items WHERE invoice_id = $1 ORDER BY sort_order",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    let payments = sqlx::query_as::<_, crate::models::payment::Payment>(
        "SELECT * FROM payments WHERE invoice_id = $1 ORDER BY collected_at DESC",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": {
            "invoice": invoice,
            "line_items": line_items,
            "payments": payments,
        },
        "meta": null,
        "errors": null,
    })))
}

async fn send_invoice(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let invoice = sqlx::query_as::<_, crate::models::invoice::Invoice>(
        r#"
        UPDATE invoices SET status = 'sent'::invoice_status, sent_at = now()
        WHERE id = $1 AND team_id = $2 AND status = 'draft'::invoice_status AND deleted_at IS NULL
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::BadRequest("Invoice must be in draft status to send".into()))?;

    // TODO: Send email/SMS with portal link
    tracing::info!(invoice_id = %id, "Invoice sent to customer");

    Ok(Json(json!({
        "data": invoice,
        "meta": { "message": "Invoice sent successfully" },
        "errors": null,
    })))
}

async fn void_invoice(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let invoice = sqlx::query_as::<_, crate::models::invoice::Invoice>(
        r#"
        UPDATE invoices SET status = 'void'::invoice_status, voided_at = now()
        WHERE id = $1 AND team_id = $2 AND status != 'void'::invoice_status AND deleted_at IS NULL
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::BadRequest("Invoice not found or already voided".into()))?;

    Ok(Json(json!({
        "data": invoice,
        "meta": { "message": "Invoice voided" },
        "errors": null,
    })))
}

async fn record_payment(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(invoice_id): Path<Uuid>,
    Json(req): Json<RecordPaymentRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let user_id = auth.id;

    // Validate payment amount
    if req.amount <= rust_decimal::Decimal::ZERO {
        return Err(ApiError::Validation("Payment amount must be greater than zero".into()));
    }

    let mut tx = state.db.begin().await?;

    // Get invoice (lock row with FOR UPDATE to prevent concurrent payment races)
    let invoice = sqlx::query_as::<_, crate::models::invoice::Invoice>(
        "SELECT * FROM invoices WHERE id = $1 AND team_id = $2 AND deleted_at IS NULL FOR UPDATE",
    )
    .bind(invoice_id)
    .bind(team_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| ApiError::NotFound("Invoice".into()))?;

    if invoice.amount_due <= rust_decimal::Decimal::ZERO {
        return Err(ApiError::BadRequest("Invoice is already fully paid".into()));
    }

    if req.amount > invoice.amount_due {
        return Err(ApiError::Validation(format!(
            "Payment amount {} exceeds amount due {}",
            req.amount, invoice.amount_due
        )));
    }

    let tip = req.tip_amount.unwrap_or_default();
    let net_amount = req.amount + tip;

    // Record payment
    let payment = sqlx::query_as::<_, crate::models::payment::Payment>(
        r#"
        INSERT INTO payments (team_id, invoice_id, customer_id, amount, tip_amount, net_amount,
                              payment_method, status, check_number, reference_number, notes, collected_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7::payment_method, 'succeeded'::payment_status, $8, $9, $10, $11)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(invoice_id)
    .bind(invoice.customer_id)
    .bind(req.amount)
    .bind(tip)
    .bind(net_amount)
    .bind(&req.payment_method)
    .bind(&req.check_number)
    .bind(&req.reference_number)
    .bind(&req.notes)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    // Update invoice amounts
    let new_amount_paid = invoice.amount_paid + req.amount;
    let new_amount_due = invoice.total - new_amount_paid;
    let new_status = if new_amount_due <= rust_decimal::Decimal::ZERO {
        "paid"
    } else {
        "partially_paid"
    };

    sqlx::query(
        r#"
        UPDATE invoices SET
            amount_paid = $3,
            amount_due = $4,
            status = $5::invoice_status,
            paid_at = CASE WHEN $5 = 'paid' THEN now() ELSE paid_at END
        WHERE id = $1 AND team_id = $2
        "#,
    )
    .bind(invoice_id)
    .bind(team_id)
    .bind(new_amount_paid)
    .bind(new_amount_due)
    .bind(new_status)
    .execute(&mut *tx)
    .await?;

    // Update customer lifetime value and outstanding balance
    sqlx::query(
        r#"
        UPDATE customers SET
            lifetime_value = lifetime_value + $2,
            outstanding_balance = outstanding_balance - $2
        WHERE id = $1
        "#,
    )
    .bind(invoice.customer_id)
    .bind(req.amount)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    tracing::info!(invoice_id = %invoice_id, amount = %req.amount, "Payment recorded");

    Ok(Json(json!({
        "data": payment,
        "meta": {
            "invoice_status": new_status,
            "amount_due": new_amount_due,
        },
        "errors": null,
    })))
}

async fn list_payments(
    State(state): State<Arc<AppState>>,
    Path(invoice_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let payments = sqlx::query_as::<_, crate::models::payment::Payment>(
        "SELECT * FROM payments WHERE invoice_id = $1 ORDER BY collected_at DESC",
    )
    .bind(invoice_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": payments,
        "meta": null,
        "errors": null,
    })))
}
