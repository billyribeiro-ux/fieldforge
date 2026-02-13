use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InventoryItem {
    pub id: Uuid,
    pub team_id: Uuid,
    pub name: String,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub unit_of_measure: String,
    pub min_stock_level: Option<Decimal>,
    pub reorder_quantity: Option<Decimal>,
    pub cost_price: Option<Decimal>,
    pub markup_pct: Option<Decimal>,
    pub sell_price: Option<Decimal>,
    pub preferred_supplier: Option<String>,
    pub photo_url: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInventoryItemRequest {
    pub name: String,
    pub sku: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub unit_of_measure: Option<String>,
    pub min_stock_level: Option<Decimal>,
    pub cost_price: Option<Decimal>,
    pub sell_price: Option<Decimal>,
    pub preferred_supplier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InventoryLocation {
    pub id: Uuid,
    pub team_id: Uuid,
    pub name: String,
    pub location_type: String,
    pub vehicle_id: Option<Uuid>,
    pub address: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InventoryStock {
    pub id: Uuid,
    pub item_id: Uuid,
    pub location_id: Uuid,
    pub quantity: Decimal,
    pub updated_at: DateTime<Utc>,
}
