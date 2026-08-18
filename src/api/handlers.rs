use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::domain:: {
    models::UserProfile, 
    password::hash_password,
};
use crate::error::AppError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserProfile>, AppError> {
    // Hash the password (CPU‑bound operation)
    let hashed_password = hash_password(&payload.password)?;

    let new_id = Uuid::new_v4();

    // Save the user in the database via the repository
    // The repository receives an already hashed string.
    let user = state
        .user_repo
        .create_user(new_id, payload.username, payload.email, hashed_password)
        .await?;



    Ok(Json(UserProfile {
        id: user.id.to_string(),
        username: user.username,
        bio: None,
    }))
}

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}