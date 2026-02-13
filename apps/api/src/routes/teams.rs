use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/team", get(get_team).patch(update_team))
        .route("/team/members", get(list_members).post(invite_member))
        .route("/team/members/{id}", patch(update_member))
        .route("/team/members/{id}/deactivate", post(deactivate_member))
}

async fn get_team(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let team = sqlx::query_as::<_, crate::models::team::Team>(
        "SELECT * FROM teams WHERE id = $1",
    )
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Team".into()))?;

    Ok(Json(json!({
        "data": team,
        "meta": null,
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct UpdateTeamRequest {
    name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    website: Option<String>,
    address_line1: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    tax_rate: Option<rust_decimal::Decimal>,
    default_hourly_rate: Option<rust_decimal::Decimal>,
}

async fn update_team(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UpdateTeamRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let team = sqlx::query_as::<_, crate::models::team::Team>(
        r#"
        UPDATE teams SET
            name = COALESCE($2, name),
            phone = COALESCE($3, phone),
            email = COALESCE($4, email),
            website = COALESCE($5, website),
            address_line1 = COALESCE($6, address_line1),
            city = COALESCE($7, city),
            state = COALESCE($8, state),
            zip_code = COALESCE($9, zip_code),
            tax_rate = COALESCE($10, tax_rate),
            default_hourly_rate = COALESCE($11, default_hourly_rate),
            updated_at = now()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(&req.name)
    .bind(&req.phone)
    .bind(&req.email)
    .bind(&req.website)
    .bind(&req.address_line1)
    .bind(&req.city)
    .bind(&req.state)
    .bind(&req.zip_code)
    .bind(req.tax_rate)
    .bind(req.default_hourly_rate)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({
        "data": team,
        "meta": null,
        "errors": null,
    })))
}

async fn list_members(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let members = sqlx::query_as::<_, crate::models::user::User>(
        "SELECT * FROM users WHERE team_id = $1 AND deleted_at IS NULL ORDER BY first_name",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": members,
        "meta": { "total": members.len() },
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct InviteMemberRequest {
    email: String,
    first_name: String,
    last_name: String,
    role: String,
    hourly_rate: Option<rust_decimal::Decimal>,
}

async fn invite_member(
    State(state): State<Arc<AppState>>,
    Json(req): Json<InviteMemberRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    // Check if email already exists
    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE email = $1",
    )
    .bind(&req.email)
    .fetch_one(&state.db)
    .await?;

    if exists > 0 {
        return Err(ApiError::Conflict("A user with this email already exists".into()));
    }

    // Create user with temporary password (they'll set their own via invite link)
    let temp_hash = crate::services::auth_service::hash_password("temp-invite-password")?;

    let user = sqlx::query_as::<_, crate::models::user::User>(
        r#"
        INSERT INTO users (team_id, email, password_hash, first_name, last_name, role, hourly_rate)
        VALUES ($1, $2, $3, $4, $5, $6::user_role, $7)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(&req.email)
    .bind(&temp_hash)
    .bind(&req.first_name)
    .bind(&req.last_name)
    .bind(&req.role)
    .bind(req.hourly_rate)
    .fetch_one(&state.db)
    .await?;

    // TODO: Send invite email with password reset link
    tracing::info!(user_id = %user.id, email = %req.email, "Team member invited");

    Ok(Json(json!({
        "data": user,
        "meta": { "message": "Invitation sent" },
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct UpdateMemberRequest {
    role: Option<String>,
    hourly_rate: Option<rust_decimal::Decimal>,
}

async fn update_member(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateMemberRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let user = sqlx::query_as::<_, crate::models::user::User>(
        r#"
        UPDATE users SET
            role = COALESCE($3::user_role, role),
            hourly_rate = COALESCE($4, hourly_rate),
            updated_at = now()
        WHERE id = $1 AND team_id = $2 AND deleted_at IS NULL
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .bind(&req.role)
    .bind(req.hourly_rate)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Team member".into()))?;

    Ok(Json(json!({
        "data": user,
        "meta": null,
        "errors": null,
    })))
}

async fn deactivate_member(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    sqlx::query(
        "UPDATE users SET is_active = false, updated_at = now() WHERE id = $1 AND team_id = $2",
    )
    .bind(id)
    .bind(team_id)
    .execute(&state.db)
    .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Team member deactivated" },
        "errors": null,
    })))
}
