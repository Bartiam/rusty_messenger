use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::models::User;
use crate::error::AppError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {
    let new_id = Uuid::new_v4();
    let user = state
        .user_repo
        .create_user(new_id, payload.username, payload.email)
        .await?;

    Ok(Json(user))
}

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}