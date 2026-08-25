use axum::{
    Json, 
    extract::State
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::{
        models::UserProfile, 
        password::hash_password
    }, error::AppError, 
    state::AppState, 
    validation::validate_input
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 40, message = "Username must be between 3 and 50 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(custom(function = "crate::validation::validate_password"))]
    pub password: String,
}

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserProfile>, AppError> {
    validate_input(&payload)?;

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
        id: user.id,
        username: user.username,
        bio: None,
    }))
}
