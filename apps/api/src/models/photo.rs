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
    pub file_key: String,
    pub filename: String,
    pub content_type: String,
    pub file_size: Option<i64>,
    pub original_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub category: String,
    pub caption: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub taken_at: Option<DateTime<Utc>>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
