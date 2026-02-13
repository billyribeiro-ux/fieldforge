use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LineItem {
    pub id: Uuid,
    pub team_id: Uuid,
    pub estimate_id: Option<Uuid>,
    pub invoice_id: Option<Uuid>,
    pub description: String,
    pub category: String,
    pub quantity: rust_decimal::Decimal,
    pub unit: String,
    pub unit_price: rust_decimal::Decimal,
    pub total: rust_decimal::Decimal,
    pub cost_price: Option<rust_decimal::Decimal>,
    pub taxable: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
