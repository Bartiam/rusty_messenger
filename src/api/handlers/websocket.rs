use axum::{extract::{Query, State, WebSocketUpgrade, ws::WebSocket}, response::IntoResponse};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::AppError, jwt::Claims, state::{self, AppState}};

#[derive(Debug, Deserialize)]
struct WebSocketQuery {
    pub token: String,
}

pub async fn websocket(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(query): Query<WebSocketQuery>,
) -> impl IntoResponse {
    let user_id = validate_token(&query.token, &state.config.jwt_secret)
        .map_err(|_| AppError::Unauthorized)?;

    ws.on_upgrade(|socket| handle_socket(socket, state, user_id))
}

async fn handle_socket(socket: WebSocket, state: AppState) {

}

fn validate_token(token: &str, secret: &str) -> Result<Uuid, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| AppError::Unauthorized)?;

    Ok(token_data.claims.sub)
}
