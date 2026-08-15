use std::sync::Arc;
use axum::extract::FromRef;
use sqlx::PgPool;

use crate::domain::repositories::UserRepository;
use crate::infrastructure::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: PgPool,
    pub user_repo: Arc<dyn UserRepository>,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        app_state.db.clone()
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(app_state: &AppState) -> Config {
        app_state.config.clone()
    }
}