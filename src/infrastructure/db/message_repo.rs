use sqlx::PgPool;
use uuid::Uuid;
use async_trait::async_trait;

use crate::{
    domain::{
        models::Message, 
        repositories::MessageRepository
    }, 
    error::AppError
};

pub struct PgMessageRepository {
    pool: PgPool
}

impl PgMessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {pool}
    }
}

#[async_trait]
impl MessageRepository for PgMessageRepository {
    async fn send_message(
        &self,
        chat_id: Uuid,
        sender_id: Uuid,
        content: &str,
    ) -> Result<Message, AppError> {
        let message = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO messages (chat_id, sender_id, content)
            VALUES ($1, $2, $3)
            RETURNING id, chat_id, sender_id, content, created_at, updated_at, deleted_at
            "#,
            chat_id,
            sender_id,
            content,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(message)
    }

    async fn get_messages(
        &self,
        chat_id: Uuid,
        limit: i64,
        offset: i64,
        include_deleted: bool,
    ) -> Result<Vec<Message>, AppError> {
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT id, chat_id, sender_id, content, created_at, updated_at, deleted_at
            FROM messages
            WHERE chat_id = $1
            AND (deleted_at IS NULL OR $2 = true)
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
            chat_id,
            include_deleted,
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(messages)
    }

    async fn delete_message(&self, message_id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE messages SET deleted_at = now() WHERE id = $1",
            message_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_user_messages_all(&self, user_id: Uuid) -> Result<Vec<Message>, AppError> {
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT id, chat_id, sender_id, content, created_at, updated_at, deleted_at
            FROM messages
            WHERE sender_id = $1
            ORDER BY created_at DESC
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(messages)
    }

    async fn is_user_in_chat(&self, user_id: Uuid, chat_id: Uuid) -> Result<bool, AppError> {
        let row = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM chat_members WHERE chat_id = $1 AND user_id = $2) AS exists",
            chat_id,
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(row.exists.unwrap_or(false))
    }
}
