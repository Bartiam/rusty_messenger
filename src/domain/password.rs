use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, 
        PasswordHasher, 
        PasswordVerifier, 
        SaltString
    },

    Argon2,
};

use crate::error::AppError;

/// Hashes the password in PHC format using Argon2id
pub fn hash_password(password: &str) -> Result<String, AppError> {
    // Generation of a cryptographically secure salt
    let salt = SaltString::generate(&mut OsRng);

    // Initialization of Argon2 with default parameters (OWASP recommendations)
    let argon2 = Argon2::default();

    // Hash calculation and formatting into a PHC string
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("Password hashing error: {}", e);
            AppError::Internal
        })?
        .to_string();

    Ok(password_hash)
}

/// Checks whether the plaintext password matches the PHC string from the database.
pub fn verify_password(
    password: &str,
    password_hash: &str
) -> Result<bool, AppError> {
    // Parsing the PHC string, extracting salt and parameters
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| {
            tracing::error!("Invalid hash format in the database: {}", e);
            AppError::Internal
        })?;
    
    // Password verification
    // If the password does not match, verify_password will return an error,
    // we turn it into false (the password is incorrect), not into a server error.
    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_valid)
}
