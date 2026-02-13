use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::post;
use axum::Router;
use serde_json::json;

use crate::AppState;

/// Stripe webhook handler — receives events from Stripe
pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/webhooks/stripe", post(handle_stripe_webhook))
}

async fn handle_stripe_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body: String,
) -> Result<StatusCode, StatusCode> {
    let _signature = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // TODO: Verify webhook signature with Stripe secret
    // let event = stripe::Webhook::construct_event(&body, signature, &webhook_secret)?;

    let event: serde_json::Value = serde_json::from_str(&body)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let event_type = event
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    tracing::info!(event_type = %event_type, "Received Stripe webhook");

    match event_type {
        "payment_intent.succeeded" => {
            if let Some(data) = event.get("data").and_then(|d| d.get("object")) {
                let payment_intent_id = data.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let amount = data.get("amount").and_then(|v| v.as_i64()).unwrap_or(0);

                tracing::info!(
                    payment_intent = %payment_intent_id,
                    amount_cents = %amount,
                    "Payment succeeded"
                );

                // Update payment status in DB
                let _result = sqlx::query(
                    r#"UPDATE payments SET status = 'succeeded'::payment_status, updated_at = NOW()
                       WHERE stripe_payment_intent_id = $1"#,
                )
                .bind(payment_intent_id)
                .execute(&state.db)
                .await;
            }
        }
        "payment_intent.payment_failed" => {
            if let Some(data) = event.get("data").and_then(|d| d.get("object")) {
                let payment_intent_id = data.get("id").and_then(|v| v.as_str()).unwrap_or("");

                tracing::warn!(payment_intent = %payment_intent_id, "Payment failed");

                let _result = sqlx::query(
                    r#"UPDATE payments SET status = 'failed'::payment_status, updated_at = NOW()
                       WHERE stripe_payment_intent_id = $1"#,
                )
                .bind(payment_intent_id)
                .execute(&state.db)
                .await;
            }
        }
        "charge.refunded" => {
            if let Some(data) = event.get("data").and_then(|d| d.get("object")) {
                let charge_id = data.get("id").and_then(|v| v.as_str()).unwrap_or("");
                tracing::info!(charge = %charge_id, "Charge refunded");
            }
        }
        "invoice.payment_succeeded" => {
            // Subscription payment succeeded
            tracing::info!("Subscription payment succeeded");
        }
        "customer.subscription.deleted" => {
            // Subscription cancelled
            tracing::info!("Subscription cancelled");
        }
        _ => {
            tracing::debug!(event_type = %event_type, "Unhandled Stripe event");
        }
    }

    Ok(StatusCode::OK)
}
