use axum::{
    http::StatusCode, 
    Json,
    Router,
    routing::get,
    routing::post,
};
use crate::domain::models::{
    CreateUserRequest, 
    UserProfile
};

pub async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserProfile>), StatusCode> {
    // In the future, there will be a call to the business logic and saving to the database

    let profile = UserProfile {
        id: "usr_123".to_string(),
        username: payload.username,
        bio: None,
    };

    Ok((StatusCode::CREATED, Json(profile)))
}

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/users", post(create_user))
}