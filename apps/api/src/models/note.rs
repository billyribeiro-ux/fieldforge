use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Note {
    pub id: Uuid,
    pub team_id: Uuid,
    pub job_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub author_id: Uuid,
    pub content: String,
    pub note_type: String,
    pub is_internal: bool,
    pub audio_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub job_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub content: String,
    pub note_type: Option<String>,
    pub is_internal: Option<bool>,
}
