use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Vehicle {
    pub id: Uuid,
    pub team_id: Uuid,
    pub assigned_to: Option<Uuid>,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub vin: Option<String>,
    pub license_plate: Option<String>,
    pub color: Option<String>,
    pub status: String,
    pub odometer: Option<i32>,
    pub registration_expiry: Option<NaiveDate>,
    pub insurance_policy: Option<String>,
    pub insurance_expiry: Option<NaiveDate>,
    pub photo_url: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVehicleRequest {
    pub make: String,
    pub model: String,
    pub year: i32,
    pub vin: Option<String>,
    pub license_plate: Option<String>,
    pub color: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub odometer: Option<i32>,
    pub registration_expiry: Option<NaiveDate>,
    pub insurance_policy: Option<String>,
    pub insurance_expiry: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct VehicleMaintenance {
    pub id: Uuid,
    pub vehicle_id: Uuid,
    pub maintenance_type: String,
    pub description: Option<String>,
    pub provider: Option<String>,
    pub cost: Option<Decimal>,
    pub odometer: Option<i32>,
    pub performed_at: NaiveDate,
    pub next_due_date: Option<NaiveDate>,
    pub next_due_odometer: Option<i32>,
    pub receipt_url: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}
