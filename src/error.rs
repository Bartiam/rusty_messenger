use axum::{
    http::StatusCode, 
    response::{ IntoResponse, Response },

};

use tracing::error;

#[derive(Debug)]
pub enum AppError {
    InvalidInput(String),
    NotFound,
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InvalidInput(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            },
            AppError::NotFound => (StatusCode::NOT_FOUND, "The resource was not found".to_string()).into_response(),
            AppError::Internal(err) => {
                // Logging a real bug for developers!
                // This will include errors in the database, network, and file system
                error!(error = %err, "Внутренняя ошибка сервера");
                // We give the client a secure plug
                (StatusCode::INTERNAL_SERVER_ERROR, "Внутренняя ошибка").into_response()
            },
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        // We treat any I/O error as an internal failure
        AppError::Internal(err.to_string())
    }
}