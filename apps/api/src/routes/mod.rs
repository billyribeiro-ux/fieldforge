pub mod audit;
pub mod auth;
pub mod customers;
pub mod estimates;
pub mod health;
pub mod inventory;
pub mod invoices;
pub mod jobs;
pub mod notes;
pub mod photos;
pub mod properties;
pub mod search;
pub mod teams;
pub mod time_entries;
pub mod webhooks;
pub mod checklists;
pub mod equipment;
pub mod expenses;
pub mod messages;
pub mod notifications;
pub mod payments;
pub mod reviews;
pub mod service_plans;
pub mod vehicles;
pub mod ws;
pub mod automation_rules;
pub mod documents;
pub mod licenses;
pub mod recurring_rules;
pub mod tags;
pub mod fuel_logs;
pub mod purchase_orders;
pub mod gps;
pub mod portal;
pub mod stripe;

use std::sync::Arc;
use axum::Router;
use crate::AppState;

pub fn api_router(_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .merge(health::router())
        .merge(auth::router())
        .merge(customers::router())
        .merge(jobs::router())
        .merge(estimates::router())
        .merge(invoices::router())
        .merge(time_entries::router())
        .merge(photos::router())
        .merge(properties::router())
        .merge(notes::router())
        .merge(inventory::router())
        .merge(vehicles::router())
        .merge(checklists::router())
        .merge(equipment::router())
        .merge(expenses::router())
        .merge(messages::router())
        .merge(reviews::router())
        .merge(service_plans::router())
        .merge(teams::router())
        .merge(search::router())
        .merge(audit::router())
        .merge(webhooks::router())
        .merge(notifications::router())
        .merge(payments::router())
        .merge(ws::router())
        .merge(automation_rules::router())
        .merge(documents::router())
        .merge(licenses::router())
        .merge(recurring_rules::router())
        .merge(tags::router())
        .merge(fuel_logs::router())
        .merge(purchase_orders::router())
        .merge(gps::router())
        .merge(portal::router())
        .merge(stripe::router())
}
