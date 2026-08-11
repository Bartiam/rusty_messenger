use serde::{
    Deserialize, 
    Serialize
};

use serde_json::json;

use axum:: {
    extract::FromRef,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};

use crate::infrastructure::config::Config;

pub enum AppError {
    InvalidInput(String),
    NotFound,
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 1. Determine the status and message for the client
        let (status, client_message) = match self {
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound => (StatusCode::NOT_FOUND, "The resource was not found".to_string()),
            AppError::Internal(_) => (
                // IMPORTANT: We ignore the internal cause of the error (_)
                // and give the client only the general phrase.
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        // 2. Creating standardized JSON
        let body = Json(json!({
            "error": client_message
        }));

        // 3. Collecting the final HTTP response
        (status, body).into_response()
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        // We treat any I/O error as an internal failure
        AppError::Internal(err.to_string())
    }
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Config,
    // pub db_pool: PgPool,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub _email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}

