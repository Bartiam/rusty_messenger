use axum::extract::State;

use crate::domain::models::AppState;

pub async fn create_user(
    State(state): State<AppState>
) -> String {
    format!("Сервер запущен на порту {}", state.config.port)
}