use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

use crate::config::Settings;

pub fn cors_layer(settings: &Settings) -> CorsLayer {
    if settings.is_production() {
        CorsLayer::new()
            .allow_origin("https://app.fieldforge.com".parse::<HeaderValue>().unwrap())
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::PATCH,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers(tower_http::cors::Any)
            .allow_credentials(true)
            .max_age(std::time::Duration::from_secs(3600))
    } else {
        CorsLayer::very_permissive()
    }
}
