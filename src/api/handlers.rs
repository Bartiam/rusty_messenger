use axum::Extension;
use axum::response::Response;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::domain:: {
    models::UserProfile, 
    password::hash_password,
};
use crate::error::AppError;
use crate::middleware::auth::CurrentUser;
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

// PLUG FUNCTION //
pub async fn create_chat_handler(
    Extension(current_user): Extension<CurrentUser>,
) -> impl IntoResponse {
    Json(json!({
        "message": "Chat created",
        "user_id": current_user.id
    }))
}

// PLUG FUNCTION //
pub async fn send_message_handler(
    Extension(current_user): Extension<CurrentUser>,
) -> impl IntoResponse {
    Json(json!({
        "message": "Message sent",
        "user_id": current_user.id
    }))
}

// PLUG FUNCTION //
pub async fn register_handler() -> impl IntoResponse {
    Json(json!({ "message": "User registered" }))
}

// PLUG FUNCTION //
pub async fn login_handler() -> impl IntoResponse {
    Json(json!({ "token": "jwt_token_example" }))
}