use axum::Json;

use crate::domain::models::AppError;

pub mod models;

async fn read_config() -> Result<Json<String>, AppError> {
    // Operator ? if it encounters std::io::Error, it will automatically trigger
    // From::from() and returns AppError::Internal
    let content = std::fs::read_to_string("config.json")?;
    Ok(Json(content))
}
