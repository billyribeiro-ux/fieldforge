use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TimeEntry {
    pub id: Uuid,
    pub team_id: Uuid,
    pub job_id: Uuid,
    pub user_id: Uuid,
    pub entry_type: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub total_cost: Option<rust_decimal::Decimal>,
    pub notes: Option<String>,
    pub latitude_start: Option<f64>,
    pub longitude_start: Option<f64>,
    pub latitude_end: Option<f64>,
    pub longitude_end: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct StartTimerRequest {
    pub job_id: Uuid,
    pub entry_type: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct StopTimerRequest {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub notes: Option<String>,
}
