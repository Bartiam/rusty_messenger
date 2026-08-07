use axum::{Json, Router, response::IntoResponse, routing::{get, post}};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    _email: String,
}

#[derive(Serialize)]
struct UserProfile {
    user_id: String,
    username: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    bio: Option<String>
}


async fn health_handler() -> impl IntoResponse {
    "OK"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/users", post(create_user));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> Json<UserProfile> {
    let response = UserProfile {
        user_id: "uuid-v4-placeholder".to_string(),
        username: payload.username,
        bio: None,
    };

    Json(response)
}