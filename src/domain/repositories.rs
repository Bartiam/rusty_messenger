use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::User;
use crate::error::AppError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, id: Uuid, username: String, email: String, password_hash: String) -> Result<User, AppError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
}
