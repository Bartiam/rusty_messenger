use axum::{
    Json, 
    extract::State, 
};
use serde::{
    Deserialize, 
    Serialize
};
use validator::Validate;

use crate::{domain::password::verify_password, error::AppError, jwt::generate_jwt, state::AppState};

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, message = "The password cannot be empty."))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub async fn login_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Looking for a user in the database via the Repository.
    let user = state.user_repo.find_by_email(&payload.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;
    
    // Checking the Argon2id password hash.
    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::InvalidCredentials);
    }

    // Generate a JWT using the secret key from the configuration.
    let token = generate_jwt(user.id, &state.config.jwt_secret)
        .map_err(|_| AppError::Internal)?;

    // Returning the token to the client
    Ok(Json(AuthResponse { token }))
}
