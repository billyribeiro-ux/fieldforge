use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Photo {
    pub id: Uuid,
    pub team_id: Uuid,
    pub job_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub uploaded_by: Uuid,
    pub original_url: String,
    pub thumbnail_url: Option<String>,
    pub medium_url: Option<String>,
    pub large_url: Option<String>,
    pub category: String,
    pub caption: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub taken_at: Option<DateTime<Utc>>,
    pub file_size_bytes: i64,
    pub mime_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}
