use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RecurringRule {
    pub id: Uuid,
    pub team_id: Uuid,
    pub customer_id: Uuid,
    pub property_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub frequency: String,
    pub interval_value: i32,
    pub day_of_week: Option<i32>,
    pub day_of_month: Option<i32>,
    pub month_of_year: Option<i32>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub next_occurrence: Option<NaiveDate>,
    pub estimated_duration_minutes: Option<i32>,
    pub assigned_to: Option<Uuid>,
    pub job_type: Option<String>,
    pub priority: String,
    pub is_active: bool,
    pub auto_schedule: bool,
    pub advance_days: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecurringRuleRequest {
    pub customer_id: Uuid,
    pub property_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub frequency: String,
    pub interval_value: Option<i32>,
    pub day_of_week: Option<i32>,
    pub day_of_month: Option<i32>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub estimated_duration_minutes: Option<i32>,
    pub assigned_to: Option<Uuid>,
    pub job_type: Option<String>,
    pub priority: Option<String>,
    pub auto_schedule: Option<bool>,
    pub advance_days: Option<i32>,
}
