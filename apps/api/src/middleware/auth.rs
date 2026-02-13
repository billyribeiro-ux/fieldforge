use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::ApiError;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub role: String,
    pub team_id: Option<Uuid>,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub team_id: Option<Uuid>,
}

pub async fn require_auth(
    State(state): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiError::Unauthorized)?;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.auth.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized)?
    .claims;

    let auth_user = AuthUser {
        id: claims.sub,
        email: claims.email,
        role: claims.role,
        team_id: claims.team_id,
    };

    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}

pub fn create_token(
    user_id: Uuid,
    email: &str,
    role: &str,
    team_id: Option<Uuid>,
    secret: &str,
    expiry_hours: i64,
) -> Result<String, ApiError> {
    let now = chrono::Utc::now();
    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        role: role.to_string(),
        team_id,
        iat: now.timestamp(),
        exp: (now + chrono::Duration::hours(expiry_hours)).timestamp(),
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| ApiError::Internal(anyhow::anyhow!("Token creation failed: {}", e)))
}
