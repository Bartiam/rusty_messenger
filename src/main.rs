use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod domain;
mod error;
mod infrastructure;
mod state;
mod jwt;
mod middleware;
mod router;

use infrastructure::config::Config;
use infrastructure::db::user_repo::PgUserRepository;
use state::AppState;

use crate::router::app_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Configuration and database
    let config = Config::from_env();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Repository and state build (DI)
    let user_repo = Arc::new(PgUserRepository::new(pool.clone()));
    let state = AppState {
        config: config.clone(),
        db: pool,
        user_repo,
    };

    let app = app_router(state);

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("The server is running on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
