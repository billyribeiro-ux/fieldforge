use std::sync::Arc;

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::ApiResult;
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/search", get(global_search))
}

#[derive(Deserialize)]
struct SearchParams {
    q: String,
    #[serde(rename = "type")]
    search_type: Option<String>,
    limit: Option<i64>,
}

async fn global_search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchParams>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();
    let limit = params.limit.unwrap_or(10).min(50);
    let query = format!("%{}%", params.q.to_lowercase());

    // Search across multiple entities using PostgreSQL ILIKE
    // In production, this would use Meilisearch for full-text search
    let search_type = params.search_type.as_deref().unwrap_or("all");

    let mut results = serde_json::Map::new();

    if search_type == "all" || search_type == "jobs" {
        let jobs = sqlx::query_as::<_, crate::models::job::Job>(
            r#"
            SELECT * FROM jobs
            WHERE team_id = $1 AND deleted_at IS NULL
              AND (LOWER(title) LIKE $2 OR LOWER(description) LIKE $2 OR po_number ILIKE $2)
            ORDER BY created_at DESC LIMIT $3
            "#,
        )
        .bind(team_id)
        .bind(&query)
        .bind(limit)
        .fetch_all(&state.db)
        .await?;

        results.insert("jobs".into(), json!(jobs));
    }

    if search_type == "all" || search_type == "customers" {
        let customers = sqlx::query_as::<_, crate::models::customer::Customer>(
            r#"
            SELECT * FROM customers
            WHERE team_id = $1 AND deleted_at IS NULL
              AND (LOWER(first_name || ' ' || last_name) LIKE $2
                   OR LOWER(email) LIKE $2
                   OR phone LIKE $2
                   OR LOWER(company_name) LIKE $2)
            ORDER BY created_at DESC LIMIT $3
            "#,
        )
        .bind(team_id)
        .bind(&query)
        .bind(limit)
        .fetch_all(&state.db)
        .await?;

        results.insert("customers".into(), json!(customers));
    }

    if search_type == "all" || search_type == "estimates" {
        let estimates = sqlx::query_as::<_, crate::models::estimate::Estimate>(
            r#"
            SELECT * FROM estimates
            WHERE team_id = $1 AND deleted_at IS NULL
              AND (LOWER(estimate_number) LIKE $2 OR LOWER(title) LIKE $2)
            ORDER BY created_at DESC LIMIT $3
            "#,
        )
        .bind(team_id)
        .bind(&query)
        .bind(limit)
        .fetch_all(&state.db)
        .await?;

        results.insert("estimates".into(), json!(estimates));
    }

    if search_type == "all" || search_type == "invoices" {
        let invoices = sqlx::query_as::<_, crate::models::invoice::Invoice>(
            r#"
            SELECT * FROM invoices
            WHERE team_id = $1 AND deleted_at IS NULL
              AND LOWER(invoice_number) LIKE $2
            ORDER BY created_at DESC LIMIT $3
            "#,
        )
        .bind(team_id)
        .bind(&query)
        .bind(limit)
        .fetch_all(&state.db)
        .await?;

        results.insert("invoices".into(), json!(invoices));
    }

    Ok(Json(json!({
        "data": results,
        "meta": { "query": params.q },
        "errors": null,
    })))
}
