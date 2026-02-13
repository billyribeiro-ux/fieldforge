pub mod auth;
pub mod customers;
pub mod jobs;
pub mod health;

use std::sync::Arc;
use axum::Router;
use crate::AppState;

pub fn api_router(_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .merge(health::router())
        .merge(auth::router())
        .merge(customers::router())
        .merge(jobs::router())
}
