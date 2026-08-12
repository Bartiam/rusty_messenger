use axum::extract::FromRef;
use clap::Parser;

use crate::domain::models::AppState;

#[derive(Parser, Debug, Clone, FromRef)]
pub struct Config {
    /// The port where the HTTP server will be running
    #[arg(long, env = "PORT", default_value_t = 3000)]
    pub port: u16,

    /// URL for connecting to the database
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: String,
}

impl Config {
    pub fn load() -> Self {
        // We are trying to load variables from the .env file, if it exists.
        // Method .ok() ignores the error if there is no file (which is normal for production).
        dotenvy::dotenv().ok();

        // Parsing environment variables into a strongly typed structure
        Self::parse()
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(app_state: &AppState) -> Config {
        app_state.config.clone()
    }
}