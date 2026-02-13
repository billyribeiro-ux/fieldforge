use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub team_id: Uuid,
    pub invoice_id: Uuid,
    pub customer_id: Uuid,
    pub amount: rust_decimal::Decimal,
    pub tip_amount: rust_decimal::Decimal,
    pub processing_fee: rust_decimal::Decimal,
    pub net_amount: rust_decimal::Decimal,
    pub payment_method: String,
    pub status: String,
    pub stripe_payment_intent_id: Option<String>,
    pub stripe_charge_id: Option<String>,
    pub check_number: Option<String>,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
    pub refunded_amount: rust_decimal::Decimal,
    pub refund_reason: Option<String>,
    pub collected_by: Option<Uuid>,
    pub collected_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RecordPaymentRequest {
    pub amount: rust_decimal::Decimal,
    pub payment_method: String,
    pub tip_amount: Option<rust_decimal::Decimal>,
    pub check_number: Option<String>,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
}
