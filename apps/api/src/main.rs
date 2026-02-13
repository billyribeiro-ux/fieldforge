#![allow(dead_code)]

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod errors;
mod middleware;
mod models;
mod routes;
mod services;

pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: config::Settings,
    pub redis: redis::aio::ConnectionManager,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fieldforge_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let settings = config::Settings::from_env()?;

    let db_pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .min_connections(settings.database.min_connections)
        .connect(&settings.database.url)
        .await?;

    sqlx::migrate!("./src/db/migrations")
        .run(&db_pool)
        .await?;

    tracing::info!("Database connected and migrations applied");

    let redis_client = redis::Client::open(settings.redis.url.as_str())?;
    let redis_conn = redis::aio::ConnectionManager::new(redis_client).await?;

    tracing::info!("Redis connected");

    let state = Arc::new(AppState {
        db: db_pool,
        config: settings.clone(),
        redis: redis_conn,
    });

    let app = Router::new()
        .nest("/api/v1", routes::api_router(state.clone()))
        .layer(axum::middleware::from_fn(middleware::rate_limit::rate_limit_middleware))
        .layer(axum::middleware::from_fn(middleware::request_id::request_id_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::cors::cors_layer(&settings))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server.port));
    tracing::info!("FieldForge API listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
