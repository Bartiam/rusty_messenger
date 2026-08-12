mod domain;
mod api;
mod infrastructure;

use axum::{Router, routing::{get, post}};
use tokio::{
    net::TcpListener,
    signal,
};
use infrastructure::config::Config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::trace::TraceLayer;

use crate::{api::handlers::create_user, domain::models::AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "rusty_messenger=debug,tower_http=debug,info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Launching the messenger server");

    let config = Config::load();

    let state = AppState {
        config
    };

    let addr = format!("0.0.0.0:{}", state.config.port);

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/users", post(create_user))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = TcpListener::bind(&addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
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