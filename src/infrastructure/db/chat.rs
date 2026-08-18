use sqlx::PgPool;
use uuid::Uuid;
use crate::error::AppError;

pub struct PgChatRepository {
    pool: PgPool,
}

impl PgChatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_group_chat(
        &self,
        name: &str,
        creator_id: Uuid,
    ) -> Result<Uuid, AppError> {
        // Start the transaction.
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let chat_id = Uuid::new_v4();

        // Creating a chat by passing &mut *tx instead of a pool.
        sqlx::query!(
            "INSERT INTO chats (id, name, is_group) VALUES ($1, $2, true)",
            chat_id,
            name,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        // Add the creator as an administrator
        sqlx::query!(
            "INSERT INTO chat_members (chat_id, user_id, role) VALUES ($1, $2, 'admin')",
            chat_id,
            creator_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

        // Explicitly record the transaction
        tx.commit().await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(chat_id)
    }
}