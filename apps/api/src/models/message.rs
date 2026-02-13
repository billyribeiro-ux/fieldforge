use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: Uuid,
    pub team_id: Uuid,
    pub customer_id: Uuid,
    pub job_id: Option<Uuid>,
    pub direction: String,
    pub channel: String,
    pub status: String,
    pub from_number: Option<String>,
    pub to_number: Option<String>,
    pub from_email: Option<String>,
    pub to_email: Option<String>,
    pub subject: Option<String>,
    pub body: String,
    pub template_id: Option<String>,
    pub external_id: Option<String>,
    pub error_message: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub clicked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub customer_id: Uuid,
    pub job_id: Option<Uuid>,
    pub channel: String,
    pub body: String,
    pub subject: Option<String>,
}
