use axum::{Json, http::StatusCode};

use crate::domain::models::{CreateUserRequest, UserProfile};

pub async fn create_user(
    Json(payload): Json<CreateUserRequest>
) -> Result<(StatusCode, Json<UserProfile>), StatusCode> {
    let profile = UserProfile {
        id: "user_123".to_string(),
        username: payload.username,
        bio: None, 
    };

    Ok((StatusCode::CREATED, Json(profile)))
}