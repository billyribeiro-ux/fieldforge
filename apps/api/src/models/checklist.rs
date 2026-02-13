use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Checklist {
    pub id: Uuid,
    pub team_id: Uuid,
    pub job_id: Option<Uuid>,
    pub template_id: Option<Uuid>,
    pub title: String,
    pub checklist_type: String,
    pub is_required: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChecklistItem {
    pub id: Uuid,
    pub checklist_id: Uuid,
    pub description: String,
    pub is_completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<Uuid>,
    pub notes: Option<String>,
    pub photo_id: Option<Uuid>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
