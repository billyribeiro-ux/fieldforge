use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::models::service_plan::{CustomerServicePlan, ServicePlan};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/service-plans", get(list_plans).post(create_plan))
        .route("/service-plans/{id}", get(get_plan).patch(update_plan).delete(delete_plan))
        .route("/service-plans/{id}/enroll", axum::routing::post(enroll_customer))
        .route("/customers/{customer_id}/service-plans", get(list_customer_plans))
}

async fn list_plans(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let plans = sqlx::query_as::<_, ServicePlan>(
        "SELECT * FROM service_plans WHERE team_id = $1 AND is_active = true ORDER BY name",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": plans,
        "meta": { "total": plans.len() },
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct CreatePlanRequest {
    name: String,
    description: Option<String>,
    price_monthly: Option<rust_decimal::Decimal>,
    price_quarterly: Option<rust_decimal::Decimal>,
    price_annual: Option<rust_decimal::Decimal>,
    visits_per_year: Option<i32>,
    included_services: Option<Vec<String>>,
    discount_pct: Option<rust_decimal::Decimal>,
    priority_scheduling: Option<bool>,
}

async fn create_plan(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreatePlanRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();
    let visits = req.visits_per_year.unwrap_or(1);
    let priority = req.priority_scheduling.unwrap_or(true);

    let plan = sqlx::query_as::<_, ServicePlan>(
        r#"
        INSERT INTO service_plans (team_id, name, description, price_monthly, price_quarterly,
                                   price_annual, visits_per_year, included_services, discount_pct, priority_scheduling)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(req.price_monthly)
    .bind(req.price_quarterly)
    .bind(req.price_annual)
    .bind(visits)
    .bind(&req.included_services)
    .bind(req.discount_pct)
    .bind(priority)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(plan_id = %plan.id, name = %req.name, "Service plan created");

    Ok(Json(json!({
        "data": plan,
        "meta": null,
        "errors": null,
    })))
}

async fn get_plan(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let plan = sqlx::query_as::<_, ServicePlan>(
        "SELECT * FROM service_plans WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Service plan".into()))?;

    let enrollments = sqlx::query_as::<_, CustomerServicePlan>(
        "SELECT * FROM customer_service_plans WHERE service_plan_id = $1 AND status = 'active'",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": {
            "plan": plan,
            "active_enrollments": enrollments.len(),
        },
        "meta": null,
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct UpdatePlanRequest {
    name: Option<String>,
    description: Option<String>,
    price_monthly: Option<rust_decimal::Decimal>,
    price_annual: Option<rust_decimal::Decimal>,
    visits_per_year: Option<i32>,
    discount_pct: Option<rust_decimal::Decimal>,
}

async fn update_plan(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdatePlanRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let plan = sqlx::query_as::<_, ServicePlan>(
        r#"
        UPDATE service_plans SET
            name = COALESCE($3, name),
            description = COALESCE($4, description),
            price_monthly = COALESCE($5, price_monthly),
            price_annual = COALESCE($6, price_annual),
            visits_per_year = COALESCE($7, visits_per_year),
            discount_pct = COALESCE($8, discount_pct),
            updated_at = now()
        WHERE id = $1 AND team_id = $2
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(req.price_monthly)
    .bind(req.price_annual)
    .bind(req.visits_per_year)
    .bind(req.discount_pct)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Service plan".into()))?;

    Ok(Json(json!({
        "data": plan,
        "meta": null,
        "errors": null,
    })))
}

async fn delete_plan(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    sqlx::query(
        "UPDATE service_plans SET is_active = false, updated_at = now() WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .execute(&state.db)
    .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Service plan deactivated" },
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct EnrollCustomerRequest {
    customer_id: Uuid,
    billing_frequency: Option<String>,
    start_date: chrono::NaiveDate,
}

async fn enroll_customer(
    State(state): State<Arc<AppState>>,
    Path(plan_id): Path<Uuid>,
    Json(req): Json<EnrollCustomerRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();
    let frequency = req.billing_frequency.as_deref().unwrap_or("monthly");

    let enrollment = sqlx::query_as::<_, CustomerServicePlan>(
        r#"
        INSERT INTO customer_service_plans (service_plan_id, customer_id, team_id, billing_frequency, start_date)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(plan_id)
    .bind(req.customer_id)
    .bind(team_id)
    .bind(frequency)
    .bind(req.start_date)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(enrollment_id = %enrollment.id, customer_id = %req.customer_id, "Customer enrolled in service plan");

    Ok(Json(json!({
        "data": enrollment,
        "meta": null,
        "errors": null,
    })))
}

async fn list_customer_plans(
    State(state): State<Arc<AppState>>,
    Path(customer_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let enrollments = sqlx::query_as::<_, CustomerServicePlan>(
        "SELECT * FROM customer_service_plans WHERE customer_id = $1 ORDER BY start_date DESC",
    )
    .bind(customer_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": enrollments,
        "meta": { "total": enrollments.len() },
        "errors": null,
    })))
}
