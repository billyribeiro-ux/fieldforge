use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServicePlan {
    pub id: Uuid,
    pub team_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price_monthly: Option<Decimal>,
    pub price_quarterly: Option<Decimal>,
    pub price_annual: Option<Decimal>,
    pub visits_per_year: i32,
    pub included_services: Option<Vec<String>>,
    pub discount_pct: Option<Decimal>,
    pub priority_scheduling: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CustomerServicePlan {
    pub id: Uuid,
    pub service_plan_id: Uuid,
    pub customer_id: Uuid,
    pub team_id: Uuid,
    pub billing_frequency: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub auto_renew: bool,
    pub stripe_subscription_id: Option<String>,
    pub status: String,
    pub visits_used: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
