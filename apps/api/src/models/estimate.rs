use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Estimate {
    pub id: Uuid,
    pub team_id: Uuid,
    pub job_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub property_id: Option<Uuid>,
    pub estimate_number: String,
    pub version: i32,
    pub status: String,
    pub title: Option<String>,
    pub scope_of_work: Option<String>,
    pub subtotal: rust_decimal::Decimal,
    pub discount_amount: rust_decimal::Decimal,
    pub discount_pct: Option<rust_decimal::Decimal>,
    pub tax_amount: rust_decimal::Decimal,
    pub tax_rate: Option<rust_decimal::Decimal>,
    pub total: rust_decimal::Decimal,
    pub deposit_required_pct: Option<rust_decimal::Decimal>,
    pub deposit_amount: Option<rust_decimal::Decimal>,
    pub margin_pct: Option<rust_decimal::Decimal>,
    pub internal_cost: Option<rust_decimal::Decimal>,
    pub valid_until: Option<NaiveDate>,
    pub payment_terms: Option<String>,
    pub warranty_terms: Option<String>,
    pub terms_and_conditions: Option<String>,
    pub customer_signature: Option<String>,
    pub signed_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub viewed_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub declined_at: Option<DateTime<Utc>>,
    pub decline_reason: Option<String>,
    pub portal_token: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEstimateRequest {
    pub customer_id: Uuid,
    pub job_id: Option<Uuid>,
    pub property_id: Option<Uuid>,
    pub title: Option<String>,
    pub scope_of_work: Option<String>,
    pub line_items: Vec<CreateLineItemInput>,
    pub discount_pct: Option<rust_decimal::Decimal>,
    pub discount_amount: Option<rust_decimal::Decimal>,
    pub valid_until: Option<NaiveDate>,
    pub payment_terms: Option<String>,
    pub warranty_terms: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLineItemInput {
    pub description: String,
    pub category: Option<String>,
    pub quantity: rust_decimal::Decimal,
    pub unit: Option<String>,
    pub unit_price: rust_decimal::Decimal,
    pub taxable: Option<bool>,
    pub sort_order: Option<i32>,
}
