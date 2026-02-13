use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::AppState;

/// Public customer portal endpoints — no auth required, token-based access
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/portal/estimates/{token}", get(get_estimate_by_token))
        .route("/portal/estimates/{token}/approve", post(approve_estimate))
        .route("/portal/estimates/{token}/decline", post(decline_estimate))
        .route("/portal/invoices/{token}", get(get_invoice_by_token))
        .route("/portal/invoices/{token}/pay", post(initiate_payment))
}

async fn get_estimate_by_token(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let estimate = sqlx::query_as::<_, crate::models::estimate::Estimate>(
        r#"UPDATE estimates SET status = 'viewed'::estimate_status, viewed_at = COALESCE(viewed_at, NOW()), updated_at = NOW()
           WHERE portal_token = $1 AND status != 'expired'::estimate_status
           RETURNING *"#,
    )
    .bind(&token)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Estimate".into()))?;

    let line_items = sqlx::query_as::<_, crate::models::line_item::LineItem>(
        "SELECT * FROM line_items WHERE estimate_id = $1 ORDER BY sort_order ASC",
    )
    .bind(estimate.id)
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

async fn approve_estimate(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
    Json(req): Json<ApproveEstimateRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let estimate = sqlx::query_as::<_, crate::models::estimate::Estimate>(
        r#"UPDATE estimates
           SET status = 'approved'::estimate_status,
               approved_at = NOW(),
               approved_by_name = $2,
               updated_at = NOW()
           WHERE portal_token = $1 AND status IN ('sent'::estimate_status, 'viewed'::estimate_status)
           RETURNING *"#,
    )
    .bind(&token)
    .bind(&req.signer_name)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Estimate".into()))?;

    tracing::info!(estimate_id = %estimate.id, "Estimate approved via portal");

    Ok(Json(json!({ "data": estimate, "meta": null, "errors": null })))
}

async fn decline_estimate(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
    Json(req): Json<DeclineEstimateRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let estimate = sqlx::query_as::<_, crate::models::estimate::Estimate>(
        r#"UPDATE estimates
           SET status = 'declined'::estimate_status,
               decline_reason = $2,
               updated_at = NOW()
           WHERE portal_token = $1 AND status IN ('sent'::estimate_status, 'viewed'::estimate_status)
           RETURNING *"#,
    )
    .bind(&token)
    .bind(&req.reason)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Estimate".into()))?;

    Ok(Json(json!({ "data": estimate, "meta": null, "errors": null })))
}

async fn get_invoice_by_token(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let invoice = sqlx::query_as::<_, crate::models::invoice::Invoice>(
        r#"UPDATE invoices SET status = 'viewed'::invoice_status, viewed_at = COALESCE(viewed_at, NOW()), updated_at = NOW()
           WHERE portal_token = $1 AND status != 'void'::invoice_status
           RETURNING *"#,
    )
    .bind(&token)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Invoice".into()))?;

    let line_items = sqlx::query_as::<_, crate::models::line_item::LineItem>(
        "SELECT * FROM line_items WHERE invoice_id = $1 ORDER BY sort_order ASC",
    )
    .bind(invoice.id)
    .fetch_all(&state.db)
    .await?;

    let payments = sqlx::query_as::<_, crate::models::payment::Payment>(
        "SELECT * FROM payments WHERE invoice_id = $1 ORDER BY collected_at DESC",
    )
    .bind(invoice.id)
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

async fn initiate_payment(
    State(state): State<Arc<AppState>>,
    Path(token): Path<String>,
    Json(req): Json<InitiatePaymentRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let invoice = sqlx::query_as::<_, crate::models::invoice::Invoice>(
        "SELECT * FROM invoices WHERE portal_token = $1 AND status NOT IN ('void'::invoice_status, 'paid'::invoice_status)",
    )
    .bind(&token)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Invoice".into()))?;

    // Record the payment
    let payment = sqlx::query_as::<_, crate::models::payment::Payment>(
        r#"INSERT INTO payments (team_id, invoice_id, amount, method, status, collected_at)
           VALUES ($1, $2, $3, $4::payment_method, 'pending'::payment_status, NOW())
           RETURNING *"#,
    )
    .bind(invoice.team_id)
    .bind(invoice.id)
    .bind(req.amount)
    .bind(req.method.as_deref().unwrap_or("card"))
    .fetch_one(&state.db)
    .await?;

    tracing::info!(invoice_id = %invoice.id, payment_id = %payment.id, amount = %req.amount, "Payment initiated via portal");

    Ok(Json(json!({
        "data": {
            "payment": payment,
            "stripe_client_secret": null,
        },
        "meta": null,
        "errors": null,
    })))
}

#[derive(Debug, serde::Deserialize)]
struct ApproveEstimateRequest {
    signer_name: Option<String>,
    signature_data: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct DeclineEstimateRequest {
    reason: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct InitiatePaymentRequest {
    amount: rust_decimal::Decimal,
    method: Option<String>,
}
