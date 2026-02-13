use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::models::message::{Message, SendMessageRequest};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/messages", get(list_messages).post(send_message))
        .route("/customers/{customer_id}/messages", get(list_customer_messages))
        .route("/jobs/{job_id}/messages", get(list_job_messages))
        .route("/messages/{id}", get(get_message))
}

async fn list_messages(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    let messages = sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE team_id = $1 ORDER BY created_at DESC LIMIT 100",
    )
    .bind(team_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": messages,
        "meta": { "total": messages.len() },
        "errors": null,
    })))
}

async fn send_message(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SendMessageRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();

    // Look up customer contact info
    let customer = sqlx::query_as::<_, crate::models::customer::Customer>(
        "SELECT * FROM customers WHERE id = $1 AND team_id = $2",
    )
    .bind(req.customer_id)
    .bind(team_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Customer".into()))?;

    let (to_number, to_email) = match req.channel.as_str() {
        "sms" => (customer.phone.clone(), None),
        "email" => (None, customer.email.clone()),
        _ => (None, None),
    };

    let message = sqlx::query_as::<_, Message>(
        r#"
        INSERT INTO messages (team_id, customer_id, job_id, direction, channel, status,
                              to_number, to_email, subject, body)
        VALUES ($1, $2, $3, 'outbound', $4::message_channel, 'queued', $5, $6, $7, $8)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(req.customer_id)
    .bind(req.job_id)
    .bind(&req.channel)
    .bind(&to_number)
    .bind(&to_email)
    .bind(&req.subject)
    .bind(&req.body)
    .fetch_one(&state.db)
    .await?;

    // TODO: Actually send via Twilio/SendGrid and update status
    tracing::info!(message_id = %message.id, channel = %req.channel, "Message queued");

    Ok(Json(json!({
        "data": message,
        "meta": null,
        "errors": null,
    })))
}

async fn list_customer_messages(
    State(state): State<Arc<AppState>>,
    Path(customer_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let messages = sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE customer_id = $1 ORDER BY created_at DESC",
    )
    .bind(customer_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": messages,
        "meta": { "total": messages.len() },
        "errors": null,
    })))
}

async fn list_job_messages(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let messages = sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE job_id = $1 ORDER BY created_at DESC",
    )
    .bind(job_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(json!({
        "data": messages,
        "meta": { "total": messages.len() },
        "errors": null,
    })))
}

async fn get_message(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let message = sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Message".into()))?;

    Ok(Json(json!({
        "data": message,
        "meta": null,
        "errors": null,
    })))
}
