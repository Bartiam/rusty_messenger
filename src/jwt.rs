use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// User identifier (UUID as a string)
    pub sub: String,
    pub exp: usize,
}

pub fn generate_jwt(user_id: &str, secret: &str) 
-> Result<String, jsonwebtoken::errors::Error> {
    // The token will be valid for 24 hours.
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Time calculation error")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    // Generate the token: default Header (HS256) + our Claims + Secret Key
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}