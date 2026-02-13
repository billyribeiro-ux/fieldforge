use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::models::license::{
    CreateInsurancePolicyRequest, CreateLicenseRequest, InsurancePolicy, License,
};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/licenses", get(list_licenses).post(create_license))
        .route("/licenses/{id}", get(get_license).delete(delete_license))
        .route(
            "/insurance-policies",
            get(list_policies).post(create_policy),
        )
        .route(
            "/insurance-policies/{id}",
            get(get_policy).delete(delete_policy),
        )
}

async fn list_licenses(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let licenses = sqlx::query_as::<_, License>(
        "SELECT * FROM licenses WHERE team_id = $1 ORDER BY expiry_date ASC",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({ "data": licenses, "meta": null, "errors": null })))
}

async fn create_license(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateLicenseRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let license = sqlx::query_as::<_, License>(
        r#"INSERT INTO licenses (team_id, user_id, license_type, license_number, issuing_state, issuing_authority, issued_date, expiry_date)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
           RETURNING *"#,
    )
    .bind(team_id)
    .bind(req.user_id)
    .bind(&req.license_type)
    .bind(&req.license_number)
    .bind(&req.issuing_state)
    .bind(&req.issuing_authority)
    .bind(req.issued_date)
    .bind(req.expiry_date)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({ "data": license, "meta": null, "errors": null })))
}

async fn get_license(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let license = sqlx::query_as::<_, License>(
        "SELECT * FROM licenses WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("License".into()))?;

    Ok(Json(json!({ "data": license, "meta": null, "errors": null })))
}

async fn delete_license(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    sqlx::query("DELETE FROM licenses WHERE id = $1 AND team_id = $2")
        .bind(id)
        .bind(team_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({ "data": null, "meta": null, "errors": null })))
}

async fn list_policies(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let policies = sqlx::query_as::<_, InsurancePolicy>(
        "SELECT * FROM insurance_policies WHERE team_id = $1 ORDER BY expiry_date ASC",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({ "data": policies, "meta": null, "errors": null })))
}

async fn create_policy(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateInsurancePolicyRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let policy = sqlx::query_as::<_, InsurancePolicy>(
        r#"INSERT INTO insurance_policies (team_id, policy_type, provider, policy_number, coverage_amount, premium_amount, effective_date, expiry_date, auto_renew, notes)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
           RETURNING *"#,
    )
    .bind(team_id)
    .bind(&req.policy_type)
    .bind(&req.provider)
    .bind(&req.policy_number)
    .bind(req.coverage_amount)
    .bind(req.premium_amount)
    .bind(req.effective_date)
    .bind(req.expiry_date)
    .bind(req.auto_renew.unwrap_or(false))
    .bind(&req.notes)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({ "data": policy, "meta": null, "errors": null })))
}

async fn get_policy(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let policy = sqlx::query_as::<_, InsurancePolicy>(
        "SELECT * FROM insurance_policies WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Insurance policy".into()))?;

    Ok(Json(json!({ "data": policy, "meta": null, "errors": null })))
}

async fn delete_policy(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    sqlx::query("DELETE FROM insurance_policies WHERE id = $1 AND team_id = $2")
        .bind(id)
        .bind(team_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({ "data": null, "meta": null, "errors": null })))
}
