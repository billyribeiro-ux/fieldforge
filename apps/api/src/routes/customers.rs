use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use axum::Extension;
use serde_json::json;
use uuid::Uuid;

use crate::db::repository;
use crate::errors::ApiResult;
use crate::middleware::auth::AuthUser;
use crate::models::common::PaginationParams;
use crate::models::customer::{CreateCustomerRequest, UpdateCustomerRequest};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/customers", get(list_customers).post(create_customer))
        .route(
            "/customers/{id}",
            get(get_customer).patch(update_customer).delete(delete_customer),
        )
    // TODO: In production, wrap with require_auth middleware layer
}

async fn create_customer(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Json(req): Json<CreateCustomerRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let customer = repository::create_customer(&state.db, team_id, &req).await?;

    Ok(Json(json!({
        "data": customer,
        "meta": null,
        "errors": null,
    })))
}

async fn list_customers(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Query(pagination): Query<PaginationParams>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let search = params.get("search").map(|s| s.as_str());
    let cursor = pagination.cursor.as_ref().and_then(|c| c.parse::<Uuid>().ok());

    let customers = repository::list_customers(&state.db, team_id, search, pagination.limit(), cursor).await?;
    let has_more = customers.len() as i64 == pagination.limit();

    Ok(Json(json!({
        "data": customers,
        "meta": {
            "has_more": has_more,
            "cursor": customers.last().map(|c| c.id.to_string()),
        },
        "errors": null,
    })))
}

async fn get_customer(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let customer = repository::get_customer(&state.db, team_id, id).await?;

    Ok(Json(json!({
        "data": customer,
        "meta": null,
        "errors": null,
    })))
}

async fn update_customer(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCustomerRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    let customer = repository::update_customer(&state.db, team_id, id, &req).await?;

    Ok(Json(json!({
        "data": customer,
        "meta": null,
        "errors": null,
    })))
}

async fn delete_customer(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = auth.team_id.unwrap_or_default();
    repository::delete_customer(&state.db, team_id, id).await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Customer deleted" },
        "errors": null,
    })))
}
