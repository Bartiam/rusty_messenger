use axum::{
    Json, 
    response::IntoResponse
};
use serde_json::json;

// PLUG FUNCTION //
pub async fn register_handler() -> impl IntoResponse {
    Json(json!({ "message": "User registered" }))
}

// PLUG FUNCTION //
pub async fn login_handler() -> impl IntoResponse {
    Json(json!({ "token": "jwt_token_example" }))
}