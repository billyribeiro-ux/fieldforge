use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct License {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Option<Uuid>,
    pub license_type: String,
    pub license_number: String,
    pub issuing_state: Option<String>,
    pub issuing_authority: Option<String>,
    pub issued_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub status: String,
    pub document_url: Option<String>,
    pub reminder_sent_90: bool,
    pub reminder_sent_60: bool,
    pub reminder_sent_30: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLicenseRequest {
    pub license_type: String,
    pub license_number: String,
    pub user_id: Option<Uuid>,
    pub issuing_state: Option<String>,
    pub issuing_authority: Option<String>,
    pub issued_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InsurancePolicy {
    pub id: Uuid,
    pub team_id: Uuid,
    pub policy_type: String,
    pub provider: String,
    pub policy_number: String,
    pub coverage_amount: Option<rust_decimal::Decimal>,
    pub premium_amount: Option<rust_decimal::Decimal>,
    pub effective_date: NaiveDate,
    pub expiry_date: NaiveDate,
    pub document_url: Option<String>,
    pub auto_renew: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInsurancePolicyRequest {
    pub policy_type: String,
    pub provider: String,
    pub policy_number: String,
    pub coverage_amount: Option<rust_decimal::Decimal>,
    pub premium_amount: Option<rust_decimal::Decimal>,
    pub effective_date: NaiveDate,
    pub expiry_date: NaiveDate,
    pub auto_renew: Option<bool>,
    pub notes: Option<String>,
}
