use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AutomationRule {
    pub id: Uuid,
    pub team_id: Uuid,
    pub name: String,
    pub trigger_event: String,
    pub conditions: serde_json::Value,
    pub actions: serde_json::Value,
    pub delay_minutes: Option<i32>,
    pub is_active: bool,
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub trigger_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAutomationRuleRequest {
    pub name: String,
    pub trigger_event: String,
    pub conditions: Option<serde_json::Value>,
    pub actions: serde_json::Value,
    pub delay_minutes: Option<i32>,
}
