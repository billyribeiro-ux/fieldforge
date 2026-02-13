use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Document {
    pub id: Uuid,
    pub team_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub file_name: String,
    pub file_url: String,
    pub file_size: i64,
    pub mime_type: String,
    pub uploaded_by: Option<Uuid>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Signature {
    pub id: Uuid,
    pub team_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub signer_name: String,
    pub signer_email: Option<String>,
    pub signature_url: String,
    pub ip_address: Option<String>,
    pub signed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FuelLog {
    pub id: Uuid,
    pub vehicle_id: Uuid,
    pub team_id: Uuid,
    pub gallons: rust_decimal::Decimal,
    pub cost_per_gallon: rust_decimal::Decimal,
    pub total_cost: rust_decimal::Decimal,
    pub odometer: Option<i32>,
    pub fuel_type: Option<String>,
    pub station: Option<String>,
    pub filled_by: Option<Uuid>,
    pub filled_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PurchaseOrder {
    pub id: Uuid,
    pub team_id: Uuid,
    pub job_id: Option<Uuid>,
    pub vendor: String,
    pub po_number: String,
    pub status: String,
    pub subtotal: rust_decimal::Decimal,
    pub tax: rust_decimal::Decimal,
    pub total: rust_decimal::Decimal,
    pub notes: Option<String>,
    pub expected_date: Option<chrono::NaiveDate>,
    pub received_date: Option<chrono::NaiveDate>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
