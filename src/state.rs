use std::sync::Arc;
use axum::extract::FromRef;
use dashmap::DashMap;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::repositories::{
    ChatRepository, 
    MessageRepository, 
    UserRepository
};
use crate::infrastructure::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: PgPool,
    pub user_repo: Arc<dyn UserRepository>,
    pub chat_repo: Arc<dyn ChatRepository>,
    pub message_repo: Arc<dyn MessageRepository>,
    pub redis: MultiplexedConnection,
    pub connections: Arc<DashMap<Uuid, tokio::sync::mpsc::UnboundedSender<axum::extract::ws::Message>>>,
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

impl FromRef<AppState> for Arc<dyn ChatRepository> {
    fn from_ref(state: &AppState) -> Self {
        state.chat_repo.clone()
    }
}
