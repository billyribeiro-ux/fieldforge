use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Property {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub team_id: Uuid,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub property_type: String,
    pub square_footage: Option<i32>,
    pub year_built: Option<i32>,
    pub stories: Option<i32>,
    pub access_notes: Option<String>,
    pub gate_code_encrypted: Option<String>,
    pub lockbox_code_encrypted: Option<String>,
    pub alarm_code_encrypted: Option<String>,
    pub pet_info: Option<String>,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePropertyRequest {
    pub customer_id: Uuid,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub property_type: Option<String>,
    pub access_notes: Option<String>,
}
