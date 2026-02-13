use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Review {
    pub id: Uuid,
    pub team_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub job_id: Option<Uuid>,
    pub platform: String,
    pub external_id: Option<String>,
    pub rating: i16,
    pub content: Option<String>,
    pub reviewer_name: Option<String>,
    pub response: Option<String>,
    pub responded_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewRequest {
    pub customer_id: Option<Uuid>,
    pub job_id: Option<Uuid>,
    pub platform: String,
    pub rating: i16,
    pub content: Option<String>,
    pub reviewer_name: Option<String>,
}
