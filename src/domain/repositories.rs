use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::{Message, User};
use crate::error::AppError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(
        &self, 
        id: Uuid, 
        username: String, 
        email: String, 
        password_hash: String
    ) -> Result<User, AppError>;

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
}

#[async_trait]
pub trait ChatRepository: Send + Sync {
    async fn create_group_chat(
        &self, 
        creator_id: Uuid, 
        name: &str, 
        initial_members: &[Uuid]
    ) -> Result<Uuid, AppError>;

    async fn get_or_create_private_chat(
        &self, 
        user1_id: Uuid, 
        user2_id: Uuid
    ) -> Result<Uuid, AppError>;
}

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn send_message(
        &self,
        chat_id: Uuid,
        sender_id: Uuid,
        content: &str,
    ) -> Result<Message, AppError>;

    async fn get_messages(
        &self,
        chat_id: Uuid,
        limit: i64,
        offset: i64,
        include_deleted: bool,
    ) -> Result<Vec<Message>, AppError>;

    // Soft deletion of the message (setting deleted_at = now())
    async fn delete_message(&self, message_id: Uuid) -> Result<(), AppError>;
    // Retrieving all user messages (including deleted ones) - for administrators
    async fn get_user_messages_all(&self, user_id: Uuid) -> Result<Vec<Message>, AppError>;
    // Checking that the user is part of the chat
    async fn is_user_in_chat(&self, user_id: Uuid, chat_id: Uuid) -> Result<bool, AppError>;
}
