use axum::{
    http::StatusCode, 
    response::{ IntoResponse, Response },
};

#[derive(Debug)]
pub enum AppError {
    InvalidInput(String),
    NotFound,
    Internal,
    InvalidCredentials,
    Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InvalidInput(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            },
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "The resource was not found".to_string()).into_response()
            }
            AppError::Internal => {
                // We give the client a secure plug
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal error").into_response()
            },
            AppError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid email or password").into_response()
            },
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Unauthorized access").into_response()
            }
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        // We treat any I/O error as an internal failure
        tracing::error!("I/O error: {:?}", err);
        AppError::Internal
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            _ => {
                tracing::error!("Database error: {:?}", err);
                AppError::Internal
            }
        }
    }
}
