use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use tokio::sync::broadcast;

use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/ws", get(ws_handler))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, _state: Arc<AppState>) {
    // Send welcome message
    if socket
        .send(Message::Text(
            serde_json::json!({
                "type": "connected",
                "message": "Connected to FieldForge real-time events"
            })
            .to_string(),
        ))
        .await
        .is_err()
    {
        return;
    }

    // Main event loop
    loop {
        tokio::select! {
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        // Handle client messages (subscribe to channels, ping, etc.)
                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                            match parsed.get("type").and_then(|t| t.as_str()) {
                                Some("ping") => {
                                    let _ = socket.send(Message::Text(
                                        serde_json::json!({"type": "pong"}).to_string()
                                    )).await;
                                }
                                Some("subscribe") => {
                                    let channel = parsed.get("channel").and_then(|c| c.as_str()).unwrap_or("default");
                                    tracing::debug!("Client subscribed to channel: {}", channel);
                                    let _ = socket.send(Message::Text(
                                        serde_json::json!({
                                            "type": "subscribed",
                                            "channel": channel
                                        }).to_string()
                                    )).await;
                                }
                                _ => {}
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }

    tracing::debug!("WebSocket connection closed");
}

/// Event types that can be broadcast to connected clients
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type")]
pub enum WsEvent {
    #[serde(rename = "job_status_changed")]
    JobStatusChanged {
        job_id: String,
        old_status: String,
        new_status: String,
    },
    #[serde(rename = "payment_received")]
    PaymentReceived {
        payment_id: String,
        invoice_id: String,
        amount: String,
    },
    #[serde(rename = "new_message")]
    NewMessage {
        message_id: String,
        customer_id: String,
        direction: String,
    },
    #[serde(rename = "schedule_changed")]
    ScheduleChanged {
        job_id: String,
        change_type: String,
    },
    #[serde(rename = "inventory_alert")]
    InventoryAlert {
        item_id: String,
        item_name: String,
        current_quantity: i32,
        min_quantity: i32,
    },
    #[serde(rename = "technician_location")]
    TechnicianLocation {
        user_id: String,
        latitude: f64,
        longitude: f64,
    },
}
