use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Expense {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Option<Uuid>,
    pub job_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
    pub category: String,
    pub description: String,
    pub amount: Decimal,
    pub tax_amount: Option<Decimal>,
    pub vendor: Option<String>,
    pub receipt_url: Option<String>,
    pub expense_date: NaiveDate,
    pub is_billable: bool,
    pub is_reimbursable: bool,
    pub reimbursed: bool,
    pub mileage: Option<Decimal>,
    pub mileage_rate: Option<Decimal>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateExpenseRequest {
    pub job_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
    pub category: String,
    pub description: String,
    pub amount: Decimal,
    pub tax_amount: Option<Decimal>,
    pub vendor: Option<String>,
    pub expense_date: NaiveDate,
    pub is_billable: Option<bool>,
    pub is_reimbursable: Option<bool>,
    pub mileage: Option<Decimal>,
    pub mileage_rate: Option<Decimal>,
    pub notes: Option<String>,
}
