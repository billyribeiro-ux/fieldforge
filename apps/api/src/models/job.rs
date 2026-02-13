use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "job_status", rename_all = "snake_case")]
pub enum JobStatus {
    Lead,
    Estimated,
    Approved,
    Scheduled,
    EnRoute,
    InProgress,
    Paused,
    Completed,
    Invoiced,
    Paid,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "job_priority", rename_all = "lowercase")]
pub enum JobPriority {
    Emergency,
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub team_id: Uuid,
    pub customer_id: Uuid,
    pub property_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub parent_job_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub job_type: Option<String>,
    pub trade: Option<String>,
    pub source: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub scheduled_start_time: Option<NaiveTime>,
    pub scheduled_end_time: Option<NaiveTime>,
    pub arrival_window_start: Option<NaiveTime>,
    pub arrival_window_end: Option<NaiveTime>,
    pub estimated_duration_minutes: Option<i32>,
    pub actual_duration_minutes: Option<i32>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub budget_amount: Option<rust_decimal::Decimal>,
    pub total_amount: Option<rust_decimal::Decimal>,
    pub internal_notes: Option<String>,
    pub access_instructions: Option<String>,
    pub permit_required: bool,
    pub warranty_job: bool,
    pub recurring_rule_id: Option<Uuid>,
    pub po_number: Option<String>,
    pub tags: Vec<String>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub customer_id: Uuid,
    pub property_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub job_type: Option<String>,
    pub trade: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub scheduled_start_time: Option<NaiveTime>,
    pub estimated_duration_minutes: Option<i32>,
    pub access_instructions: Option<String>,
    pub internal_notes: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateJobRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub scheduled_date: Option<NaiveDate>,
    pub scheduled_start_time: Option<NaiveTime>,
    pub scheduled_end_time: Option<NaiveTime>,
    pub estimated_duration_minutes: Option<i32>,
    pub internal_notes: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct JobStatusTransition {
    pub status: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct JobListItem {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub customer_first_name: String,
    pub customer_last_name: String,
    pub title: String,
    pub status: String,
    pub priority: String,
    pub scheduled_date: Option<NaiveDate>,
    pub scheduled_start_time: Option<NaiveTime>,
    pub assigned_to: Option<Uuid>,
    pub total_amount: Option<rust_decimal::Decimal>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct JobFilters {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub trade: Option<String>,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub search: Option<String>,
}
