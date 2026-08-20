use axum::{Json, extract::State};
use std::sync::Arc;
use serde::{Serialize, Deserialize};

use crate::{domain::password::verify_password, error::AppError, jwt::generate_jwt, state::AppState};


#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub async fn login_user(
    State(state): State<Arc<AppState>>,
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
    let token = generate_jwt(&user.id.to_string(), &state.config.jwt_secret)
        .map_err(|_| AppError::Internal)?;

    // Returning the token to the client
    Ok(Json(AuthResponse { token }))
}
