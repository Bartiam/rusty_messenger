use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION, 
    middleware::Next, 
    response::Response
};
use jsonwebtoken::{
    DecodingKey, 
    Validation, 
    decode
};

use uuid::Uuid;

use crate::{error::AppError, infrastructure::config::Config, jwt::Claims};

#[derive(Clone)]
pub struct CurrentUser {
    pub id: Uuid,
}

fn extract_token(req: &Request) -> Result<String, AppError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized);
    }

    Ok(auth_header[7..].to_string())
}

pub async fn auth_middleware(
    State(config): State<Config>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extracting the token
    let token = extract_token(&req)?;

    // Validate the JWT
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&config.jwt_secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| AppError::Unauthorized)?;

    // Save the user ID in the request context
    let current_user = CurrentUser {id: token_data.claims.sub};
    req.extensions_mut().insert(current_user);

    // Passing control on
    Ok(next.run(req).await)
}