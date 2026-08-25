use axum::{
    extract::{
        Request, 
        State
    },
    http::header::AUTHORIZATION, 
    middleware::Next, 
    response::Response
};
use jsonwebtoken::{
    DecodingKey, 
    Validation, 
    decode
};

use redis::{
    AsyncCommands, 
    aio::MultiplexedConnection
};
use uuid::Uuid;

use crate::{
    error::AppError, 
    jwt::Claims, 
    state::AppState
};

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
    State(mut state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extracting the token
    let token = extract_token(&req)?;

    // Validate the JWT
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&state.config.jwt_secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| AppError::Unauthorized)?;

    let is_revoked: bool = state
        .redis
        .exists(format!("revoked:{}", token_data.claims.jti))
        .await
        .unwrap_or(false);

    if is_revoked {
        return Err(AppError::Unauthorized);
    }

    // Save the user ID in the request context
    let current_user = CurrentUser {id: token_data.claims.sub};
    req.extensions_mut().insert(current_user);

    // Passing control on
    Ok(next.run(req).await)
}

pub async fn revoke_token(
    mut redis: MultiplexedConnection,
    token_id: &str,
    ttl_seconds: u64,
) -> Result<(), AppError> {
    // Set the key with the value “1” and the lifetime (EX)
    let _: () = redis.set_ex(
        format!("revoked:{}", token_id), "1", ttl_seconds)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(())
}
