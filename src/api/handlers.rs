use axum::{
    http::StatusCode, 
    response::IntoResponse
};

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub mod auth;
pub mod users;
pub mod messages;
pub mod chats;