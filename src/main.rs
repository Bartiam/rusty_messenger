use axum::{
    Json, 
    Router, 
    routing::{get, post}
};

use serde::{
    Deserialize, 
    Serialize
};

use tokio::{
    net::TcpListener,
    signal,
};

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
    bin: Option<String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async { "OK "}))
        .route("/users", post(create_user));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn create_user(
    Json(payload): Json<CreateUserRequest>
) -> Json<UserProfile> {
    let response = UserProfile {
        user_id: "sdfsf".to_string(),
        username: payload.username,
        bin: None,
    };

    Json(response)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install the Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install the SIGTERM handler")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("The SIGINT signal (Ctrl+C) has been received, and we are starting a smooth stop...");
        },

        _ = terminate => {
            println!("A SIGTERM signal has been received, and we are starting a smooth stop...");
        },
    };
}