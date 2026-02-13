use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub team_id: Option<Uuid>,
    pub trade: Option<String>,
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub is_active: bool,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub trade: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub refresh_token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub team_id: Option<Uuid>,
    pub trade: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
