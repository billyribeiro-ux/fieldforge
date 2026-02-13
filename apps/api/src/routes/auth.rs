use std::sync::Arc;

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::json;
use uuid::Uuid;

use crate::db::repository;
use crate::errors::{ApiError, ApiResult};
use crate::middleware::auth::create_token;
use crate::models::user::{CreateUserRequest, LoginRequest};
use crate::services::auth_service;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", axum::routing::get(me))
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateUserRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    // Validate at least one identifier
    if req.email.is_none() && req.phone.is_none() {
        return Err(ApiError::Validation("Email or phone is required".into()));
    }

    // Check for existing user
    if let Some(ref email) = req.email {
        if repository::find_user_by_email(&state.db, email).await?.is_some() {
            return Err(ApiError::Conflict("Email already registered".into()));
        }
    }
    if let Some(ref phone) = req.phone {
        if repository::find_user_by_phone(&state.db, phone).await?.is_some() {
            return Err(ApiError::Conflict("Phone already registered".into()));
        }
    }

    let password_hash = auth_service::hash_password(&req.password)?;
    let user = repository::create_user(&state.db, &req, &password_hash).await?;

    let token = create_token(
        user.id,
        user.email.as_deref().unwrap_or(""),
        &format!("{:?}", user.role),
        user.team_id,
        &state.config.auth.jwt_secret,
        state.config.auth.jwt_expiry_hours,
    )?;

    let refresh_token = Uuid::new_v4().to_string();

    Ok(Json(json!({
        "data": {
            "token": token,
            "refresh_token": refresh_token,
            "user": {
                "id": user.id,
                "email": user.email,
                "phone": user.phone,
                "first_name": user.first_name,
                "last_name": user.last_name,
                "role": format!("{:?}", user.role),
                "team_id": user.team_id,
            }
        },
        "meta": null,
        "errors": null,
    })))
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let user = if let Some(ref email) = req.email {
        repository::find_user_by_email(&state.db, email).await?
    } else if let Some(ref phone) = req.phone {
        repository::find_user_by_phone(&state.db, phone).await?
    } else {
        return Err(ApiError::Validation("Email or phone is required".into()));
    };

    let user = user.ok_or(ApiError::Unauthorized)?;

    if !auth_service::verify_password(&req.password, &user.password_hash)? {
        return Err(ApiError::Unauthorized);
    }

    let token = create_token(
        user.id,
        user.email.as_deref().unwrap_or(""),
        &format!("{:?}", user.role),
        user.team_id,
        &state.config.auth.jwt_secret,
        state.config.auth.jwt_expiry_hours,
    )?;

    let refresh_token = Uuid::new_v4().to_string();

    // Update last login
    sqlx::query("UPDATE users SET last_login_at = now() WHERE id = $1")
        .bind(user.id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({
        "data": {
            "token": token,
            "refresh_token": refresh_token,
            "user": {
                "id": user.id,
                "email": user.email,
                "phone": user.phone,
                "first_name": user.first_name,
                "last_name": user.last_name,
                "role": format!("{:?}", user.role),
                "team_id": user.team_id,
            }
        },
        "meta": null,
        "errors": null,
    })))
}

async fn me(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> ApiResult<Json<serde_json::Value>> {
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Unauthorized)?;

    let claims = crate::middleware::auth::verify_token(token, &state.config.auth.jwt_secret)
        .map_err(|_| ApiError::Unauthorized)?;

    let user = sqlx::query_as::<_, crate::models::user::User>(
        "SELECT * FROM users WHERE id = $1",
    )
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::Unauthorized)?;

    Ok(Json(json!({
        "data": {
            "id": user.id,
            "email": user.email,
            "phone": user.phone,
            "first_name": user.first_name,
            "last_name": user.last_name,
            "role": format!("{:?}", user.role),
            "team_id": user.team_id,
            "avatar_url": user.avatar_url,
        },
        "meta": null,
        "errors": null,
    })))
}
