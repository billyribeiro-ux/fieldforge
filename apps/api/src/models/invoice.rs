use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Invoice {
    pub id: Uuid,
    pub team_id: Uuid,
    pub job_id: Option<Uuid>,
    pub estimate_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub property_id: Option<Uuid>,
    pub invoice_number: String,
    pub status: String,
    pub subtotal: rust_decimal::Decimal,
    pub discount_amount: rust_decimal::Decimal,
    pub tax_amount: rust_decimal::Decimal,
    pub tax_rate: Option<rust_decimal::Decimal>,
    pub total: rust_decimal::Decimal,
    pub amount_paid: rust_decimal::Decimal,
    pub amount_due: rust_decimal::Decimal,
    pub due_date: Option<NaiveDate>,
    pub payment_terms: Option<String>,
    pub late_fee_amount: Option<rust_decimal::Decimal>,
    pub late_fee_applied: bool,
    pub notes: Option<String>,
    pub terms_and_conditions: Option<String>,
    pub po_number: Option<String>,
    pub portal_token: String,
    pub pdf_url: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub viewed_at: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
    pub voided_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInvoiceRequest {
    pub customer_id: Uuid,
    pub job_id: Option<Uuid>,
    pub estimate_id: Option<Uuid>,
    pub property_id: Option<Uuid>,
    pub line_items: Vec<super::estimate::CreateLineItemInput>,
    pub due_date: Option<NaiveDate>,
    pub payment_terms: Option<String>,
    pub notes: Option<String>,
}
