use axum::{extract::State, response::IntoResponse, http::StatusCode};
use sqlx::PgPool;

use crate::infrastructure::config::Config;

pub async fn create_user(
    State(config): State<Config>,
) -> String {
    format!("Сервер запущен на порту: {}", config.port)
}

pub async fn db_health_check(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, StatusCode> {
    let result: Result<i32, _> = sqlx::query_scalar("SELECT 1")
        .fetch_one(&pool)
        .await;

    match result {
        Ok(1) => Ok((StatusCode::OK, "The connection to the database is stable.")),
        _ => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}