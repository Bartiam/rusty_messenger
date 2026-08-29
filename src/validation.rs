use validator::{
    ValidationError, 
    ValidationErrors
};

use crate::error::AppError;

/// Checks password complexity: at least 8 characters, 
/// includes a capital letter, a lowercase letter, and a digit.
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }

    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());

    if has_upper && has_lower && has_digit {
        Ok(())
    }
    else {
        Err(ValidationError::new("incorrect_password_complexity"))
    }
}

/// A universal function for validating any types that implement 'Validate'.
pub fn validate_input<T: validator::Validate> (data: &T) -> Result<(), AppError> {
    if let Err(errors) = data.validate() {
        let error_messages = format_validation_errors(&errors);
        return Err(AppError::InvalidInput(error_messages));
    }

    Ok(())
}

/// Formats validation errors into a single line for convenient display to the client.
pub fn format_validation_errors(errors: &ValidationErrors) -> String {
    let mut messages = Vec::new();
    for (field, field_errors) in errors.field_errors() {
        let field_msgs: Vec<String> = field_errors
            .iter()
            .filter_map(|e| {
                // Take the message if it is provided; otherwise, use the error code
                e.message
                    .clone()
                    .map(|msg| msg.to_string())
                    .or_else(|| Some(e.code.to_string()))  
            })
            .collect();
        messages.push(format!("{}: {}", field, field_msgs.join(", ")));
    }
    messages.join("; ")
}
