mod domain;
mod api;
mod infrastructure;

use tokio::{
    net::TcpListener,
    signal,
};

use infrastructure::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::load();

    println!("Запуск сервера на порту: {}", config.port);

    let app = api::handlers::create_router();
    let addr = format!("0.0.0.0:{}", config.port);

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