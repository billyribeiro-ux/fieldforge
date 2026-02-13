pub mod auth;
pub mod customers;
pub mod estimates;
pub mod health;
pub mod invoices;
pub mod jobs;
pub mod notes;
pub mod photos;
pub mod properties;
pub mod teams;
pub mod time_entries;

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
        .merge(teams::router())
}
