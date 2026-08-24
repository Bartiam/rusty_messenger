use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::User;
use crate::error::AppError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, id: Uuid, username: String, email: String, password_hash: String) -> Result<User, AppError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
}

#[async_trait]
pub trait ChatRepository: Send + Sync {
    async fn create_group_chat(&self, creator_id: Uuid, name: &str, initial_members: &[Uuid]) -> Result<Uuid, AppError>;
    async fn get_or_create_private_chat(&self, user1_id: Uuid, user2_id: Uuid) -> Result<Uuid, AppError>;
}
