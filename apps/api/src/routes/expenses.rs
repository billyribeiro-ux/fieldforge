use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use axum::Extension;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::middleware::auth::AuthUser;
use crate::models::expense::{CreateExpenseRequest, Expense};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/expenses", get(list_expenses).post(create_expense))
        .route("/expenses/{id}", get(get_expense).patch(update_expense).delete(delete_expense))
        .route("/jobs/{job_id}/expenses", get(list_job_expenses))
}

#[derive(Deserialize)]
struct ExpenseFilters {
    category: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
    is_billable: Option<bool>,
}

async fn list_expenses(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Query(_filters): Query<ExpenseFilters>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let expenses = sqlx::query_as::<_, Expense>(
        "SELECT * FROM expenses WHERE team_id = $1 ORDER BY expense_date DESC",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    let total: rust_decimal::Decimal = expenses.iter().map(|e| e.amount).sum();

    Ok(Json(json!({
        "data": expenses,
        "meta": { "total": expenses.len(), "total_amount": total },
        "errors": null,
    })))
}

async fn create_expense(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(req): Json<CreateExpenseRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let user_id = auth.id;

    let is_billable = req.is_billable.unwrap_or(false);
    let is_reimbursable = req.is_reimbursable.unwrap_or(false);

    let expense = sqlx::query_as::<_, Expense>(
        r#"
        INSERT INTO expenses (team_id, user_id, job_id, vehicle_id, category, description, amount,
                              tax_amount, vendor, expense_date, is_billable, is_reimbursable,
                              mileage, mileage_rate, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(user_id)
    .bind(req.job_id)
    .bind(req.vehicle_id)
    .bind(&req.category)
    .bind(&req.description)
    .bind(req.amount)
    .bind(req.tax_amount)
    .bind(&req.vendor)
    .bind(req.expense_date)
    .bind(is_billable)
    .bind(is_reimbursable)
    .bind(req.mileage)
    .bind(req.mileage_rate)
    .bind(&req.notes)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(expense_id = %expense.id, amount = %req.amount, "Expense created");

    Ok(Json(json!({
        "data": expense,
        "meta": null,
        "errors": null,
    })))
}

async fn get_expense(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let expense = sqlx::query_as::<_, Expense>(
        "SELECT * FROM expenses WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Expense".into()))?;

    Ok(Json(json!({
        "data": expense,
        "meta": null,
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct UpdateExpenseRequest {
    category: Option<String>,
    description: Option<String>,
    amount: Option<rust_decimal::Decimal>,
    vendor: Option<String>,
    notes: Option<String>,
    is_billable: Option<bool>,
    reimbursed: Option<bool>,
}

async fn update_expense(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateExpenseRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    let expense = sqlx::query_as::<_, Expense>(
        r#"
        UPDATE expenses SET
            category = COALESCE($3, category),
            description = COALESCE($4, description),
            amount = COALESCE($5, amount),
            vendor = COALESCE($6, vendor),
            notes = COALESCE($7, notes),
            is_billable = COALESCE($8, is_billable),
            reimbursed = COALESCE($9, reimbursed),
            updated_at = now()
        WHERE id = $1 AND team_id = $2
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .bind(&req.category)
    .bind(&req.description)
    .bind(req.amount)
    .bind(&req.vendor)
    .bind(&req.notes)
    .bind(req.is_billable)
    .bind(req.reimbursed)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Expense".into()))?;

    Ok(Json(json!({
        "data": expense,
        "meta": null,
        "errors": null,
    })))
}

async fn delete_expense(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();

    sqlx::query("DELETE FROM expenses WHERE id = $1 AND team_id = $2")
        .bind(id)
        .bind(team_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Expense deleted" },
        "errors": null,
    })))
}

async fn list_job_expenses(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let expenses = sqlx::query_as::<_, Expense>(
        "SELECT * FROM expenses WHERE job_id = $1 ORDER BY expense_date DESC",
    )
    .bind(job_id)
    .fetch_all(&state.db)
    .await?;

    let total: rust_decimal::Decimal = expenses.iter().map(|e| e.amount).sum();

    Ok(Json(json!({
        "data": expenses,
        "meta": { "total": expenses.len(), "total_amount": total },
        "errors": null,
    })))
}
