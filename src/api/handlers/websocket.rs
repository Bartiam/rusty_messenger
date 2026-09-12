use axum::{extract::{State, WebSocketUpgrade, ws::WebSocket}, response::IntoResponse};

use crate::state::{self, AppState};

pub async fn websocket(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {

}
