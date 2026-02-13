use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub team_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub phone_secondary: Option<String>,
    pub company_name: Option<String>,
    pub preferred_contact_method: String,
    pub preferred_technician_id: Option<Uuid>,
    pub credit_terms: String,
    pub tax_exempt: bool,
    pub do_not_service: bool,
    pub do_not_service_reason: Option<String>,
    pub referral_source: Option<String>,
    pub referred_by_customer_id: Option<Uuid>,
    pub notes_pinned: Option<String>,
    pub tags: Vec<String>,
    pub portal_token: Option<String>,
    pub lifetime_value: rust_decimal::Decimal,
    pub outstanding_balance: rust_decimal::Decimal,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCustomerRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub referral_source: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCustomerRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub phone_secondary: Option<String>,
    pub company_name: Option<String>,
    pub preferred_contact_method: Option<String>,
    pub credit_terms: Option<String>,
    pub tax_exempt: Option<bool>,
    pub notes_pinned: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CustomerListItem {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub lifetime_value: rust_decimal::Decimal,
    pub outstanding_balance: rust_decimal::Decimal,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}
