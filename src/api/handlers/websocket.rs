use axum::{
    extract::{
        Query, 
        State, 
        WebSocketUpgrade, 
        ws::{
            WebSocket, 
            Message
        }}, 
        response::IntoResponse
    };
use futures_util::{
    SinkExt, 
    StreamExt
};
use jsonwebtoken::{
    decode, 
    DecodingKey, 
    Validation
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::AppError, 
    jwt::Claims, 
    state::AppState
};

#[derive(Debug, Deserialize)]
pub struct WebSocketQuery {
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

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state, user_id)))
}

async fn handle_socket(socket: WebSocket, state: AppState, user_id: Uuid) {
    let (mut sender, mut receiver) = socket.split();

    // Create a channel for sending messages from other tasks
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // Save the tx to the shared storage
    state.connections.insert(user_id, tx);

    // Task for sending messages via WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Processing incoming messages
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => {
                handle_incoming_message(&state, user_id, text.to_string()).await;
            }
            Message::Close(_) => {
                tracing::info!("User {} disconnected", user_id);
            }
            Message::Ping(data) => {
                tracing::trace!("Ping from {}: {:?}", user_id, data);
            }
            _ => {}
        }
    }

    // Connection closed – removing from storage
    state.connections.remove(&user_id);
    send_task.abort();
}

async fn handle_incoming_message(state: &AppState, user_id: Uuid, text: String) {
    let msg: WebSocketMessage = match serde_json::from_str(&text) {
        Ok(m) => m,
        Err(e) => {
            tracing::warn!("Failed to parse WS message from {}: {:?}. Raw: {}", user_id, e, text);
            return;
        },
    };

    tracing::info!("WS message from {} to chat {}: {}", user_id, msg.chat_id, msg.content);

    let is_member = state
        .message_repo
        .is_user_in_chat(user_id, msg.chat_id)
        .await
        .unwrap_or(false);

    if !is_member {
        tracing::warn!("User {} is not a member of chat {}", user_id, msg.chat_id);
        return;
    }

    // Save to the database
    let saved = match state.message_repo.send_message(msg.chat_id, user_id, &msg.content).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("send_message error: {:?}", e);
            return;
        },
    };

    // Get all members from chat
    let members = match state.chat_repo.get_chat_members(msg.chat_id).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("get_chat_members error: {:?}", e);
            return;
        },
    };

    tracing::info!("Broadcasting to {} members", members.len());

    // Serialize the message once
    let payload = match serde_json::to_string(&saved) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("serialize error: {:?}", e);
            return;
        },
    };

    // Send to all participants with an active connection
    for member_id in members {
        if let Some(tx) = state.connections.get(&member_id) {
            if let Err(e) = tx.send(Message::Text(payload.clone().into())) {
                tracing::warn!("Failed to send to {}: {:?}", member_id, e);
            }
        } else {
            tracing::debug!("Member {} has no active connection", member_id);
        }
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
