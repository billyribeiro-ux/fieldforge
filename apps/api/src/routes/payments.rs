use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiResult;
use crate::models::common::PaginationParams;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/payments", get(list_payments))
        .route("/payments/{id}", get(get_payment))
        .route("/payments/{id}/refund", axum::routing::post(refund_payment))
}

async fn list_payments(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();
    let limit = pagination.limit();

    let payments = sqlx::query_as::<_, crate::models::payment::Payment>(
        r#"SELECT * FROM payments WHERE team_id = $1 ORDER BY collected_at DESC LIMIT $2"#,
    )
    .bind(team_id)
    .bind(limit)
    .fetch_all(&state.db)
    .await?;

    let has_more = payments.len() as i64 == limit;

    Ok(Json(json!({
        "data": payments,
        "meta": {
            "has_more": has_more,
            "cursor": payments.last().map(|p| p.id.to_string()),
        },
        "errors": null,
    })))
}

async fn get_payment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let payment = sqlx::query_as::<_, crate::models::payment::Payment>(
        r#"SELECT * FROM payments WHERE id = $1 AND team_id = $2"#,
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| crate::errors::ApiError::NotFound("Payment".into()))?;

    Ok(Json(json!({
        "data": payment,
        "meta": null,
        "errors": null,
    })))
}

async fn refund_payment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<RefundRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let payment = sqlx::query_as::<_, crate::models::payment::Payment>(
        r#"SELECT * FROM payments WHERE id = $1 AND team_id = $2"#,
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| crate::errors::ApiError::NotFound("Payment".into()))?;

    if payment.status == "refunded" {
        return Err(crate::errors::ApiError::Validation("Payment already refunded".into()));
    }

    let refund_amount = req.amount.unwrap_or(payment.amount);

    let updated = sqlx::query_as::<_, crate::models::payment::Payment>(
        r#"UPDATE payments
           SET status = 'refunded', refunded_amount = $3, refund_reason = $4, updated_at = NOW()
           WHERE id = $1 AND team_id = $2
           RETURNING *"#,
    )
    .bind(id)
    .bind(team_id)
    .bind(refund_amount)
    .bind(&req.reason)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(payment_id = %id, amount = %refund_amount, "Payment refunded");

    Ok(Json(json!({
        "data": updated,
        "meta": null,
        "errors": null,
    })))
}

#[derive(Debug, serde::Deserialize)]
struct RefundRequest {
    amount: Option<rust_decimal::Decimal>,
    reason: Option<String>,
}
