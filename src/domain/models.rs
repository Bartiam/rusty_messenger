use serde::{
    Deserialize, 
    Serialize
};

use axum::extract::FromRef;

use crate::infrastructure::config::Config;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Config,
    // pub db_pool: PgPool,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub _email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}

