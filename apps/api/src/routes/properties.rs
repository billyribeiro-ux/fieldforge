use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::models::property::CreatePropertyRequest;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/customers/{customer_id}/properties", get(list_customer_properties).post(create_property))
        .route("/properties/{id}", get(get_property).patch(update_property).delete(delete_property))
}

async fn create_property(
    State(state): State<Arc<AppState>>,
    Path(customer_id): Path<Uuid>,
    Json(req): Json<CreatePropertyRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let property_type = req.property_type.as_deref().unwrap_or("residential");

    let property = sqlx::query_as::<_, crate::models::property::Property>(
        r#"
        INSERT INTO properties (customer_id, team_id, address_line1, address_line2, city, state, zip_code,
                                property_type, access_notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8::property_type, $9)
        RETURNING *
        "#,
    )
    .bind(customer_id)
    .bind(team_id)
    .bind(&req.address_line1)
    .bind(&req.address_line2)
    .bind(&req.city)
    .bind(&req.state)
    .bind(&req.zip_code)
    .bind(property_type)
    .bind(&req.access_notes)
    .fetch_one(&state.db)
    .await?;

    tracing::info!(property_id = %property.id, customer_id = %customer_id, "Property created");

    Ok(Json(json!({
        "data": property,
        "meta": null,
        "errors": null,
    })))
}

async fn list_customer_properties(
    State(state): State<Arc<AppState>>,
    Path(customer_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let properties = sqlx::query_as::<_, crate::models::property::Property>(
        "SELECT * FROM properties WHERE customer_id = $1 AND team_id = $2 ORDER BY is_primary DESC, created_at",
    )
    .bind(customer_id)
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": properties,
        "meta": { "total": properties.len() },
        "errors": null,
    })))
}

async fn get_property(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let property = sqlx::query_as::<_, crate::models::property::Property>(
        "SELECT * FROM properties WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Property".into()))?;

    Ok(Json(json!({
        "data": property,
        "meta": null,
        "errors": null,
    })))
}

#[derive(serde::Deserialize)]
struct UpdatePropertyRequest {
    address_line1: Option<String>,
    address_line2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    property_type: Option<String>,
    square_footage: Option<i32>,
    year_built: Option<i32>,
    stories: Option<i32>,
    access_notes: Option<String>,
    pet_info: Option<String>,
    is_primary: Option<bool>,
}

async fn update_property(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdatePropertyRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let property = sqlx::query_as::<_, crate::models::property::Property>(
        r#"
        UPDATE properties SET
            address_line1 = COALESCE($3, address_line1),
            address_line2 = COALESCE($4, address_line2),
            city = COALESCE($5, city),
            state = COALESCE($6, state),
            zip_code = COALESCE($7, zip_code),
            property_type = COALESCE($8::property_type, property_type),
            square_footage = COALESCE($9, square_footage),
            year_built = COALESCE($10, year_built),
            stories = COALESCE($11, stories),
            access_notes = COALESCE($12, access_notes),
            pet_info = COALESCE($13, pet_info),
            is_primary = COALESCE($14, is_primary),
            updated_at = now()
        WHERE id = $1 AND team_id = $2
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .bind(&req.address_line1)
    .bind(&req.address_line2)
    .bind(&req.city)
    .bind(&req.state)
    .bind(&req.zip_code)
    .bind(&req.property_type)
    .bind(req.square_footage)
    .bind(req.year_built)
    .bind(req.stories)
    .bind(&req.access_notes)
    .bind(&req.pet_info)
    .bind(req.is_primary)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Property".into()))?;

    Ok(Json(json!({
        "data": property,
        "meta": null,
        "errors": null,
    })))
}

async fn delete_property(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    sqlx::query("DELETE FROM properties WHERE id = $1 AND team_id = $2")
        .bind(id)
        .bind(team_id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Property deleted" },
        "errors": null,
    })))
}
