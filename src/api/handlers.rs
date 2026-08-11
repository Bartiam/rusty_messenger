use axum::extract::State;

use crate::infrastructure::config::Config;

pub async fn create_user(
    State(config): State<Config>,
) -> String {
    format!("Сервер запущен на порту: {}", config.port)
}