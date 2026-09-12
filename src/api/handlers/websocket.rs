use axum::{extract::{Query, State, WebSocketUpgrade, ws::{WebSocket, Message}}, response::IntoResponse};
use futures_util::{SinkExt, StreamExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::AppError, jwt::Claims, state::AppState};

#[derive(Debug, Deserialize)]
struct WebSocketQuery {
    pub token: String,
}

#[derive(Debug, Deserialize)]
struct WebSocketMessage {
    chat_id: Uuid,
    content: String,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(query): Query<WebSocketQuery>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = validate_token(&query.token, &state.config.jwt_secret)?;

    Ok(ws.on_upgrade(|socket| handle_socket(socket, state, user_id)))
}

async fn handle_socket(socket: WebSocket, state: AppState, user_id: Uuid) {
    let (mut sender, mut receiver) = socket.split();

    // Create a channel for sending messages from other tasks
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // Sokhranyayem tx v obshcheye khranilishche

    // Save the tx to the shared storage
    state.connections.insert(user_id, tx);

    // Task for sending messages via WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Processing incoming messages
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            // Processing a text message (JSON parsing, saving to the database, broadcasting)
            handle_incoming_message(&state, user_id, text.to_string()).await;
        }
    }

    // Connection closed – removing from storage
    state.connections.remove(&user_id);
    send_task.abort();
}

async fn handle_incoming_message(state: &AppState, user_id: Uuid, text: String) {
    let msg: WebSocketMessage = match serde_json::from_str(&text) {
        Ok(m) => m,
        Err(_) => return,
    };

    // Save to the database
    let saved = match state.message_repo.send_message(msg.chat_id, user_id, &msg.content).await {
        Ok(m) => m,
        Err(_) => return,
    };

    // Get all chat participants
    // Temporarily just sending it back to the sender (for testing).
    if let Some(tx) = state.connections.get(&user_id) {
        let text = serde_json::to_string(&saved).unwrap();
        let _ = tx.send(Message::Text(text.into()));
    }
}

fn validate_token(token: &str, secret: &str) -> Result<Uuid, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| AppError::Unauthorized)?;

    Ok(token_data.claims.sub)
}
