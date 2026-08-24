use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    domain::repositories::ChatRepository, 
    error::AppError
};

pub struct PgChatRepository {
    pool: PgPool,
}

impl PgChatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChatRepository for PgChatRepository {
    async fn create_group_chat(
        &self,
        creator_id: Uuid,
        name: &str,
        initial_members: &[Uuid],
    ) -> Result<Uuid, AppError> {
        // Start the transaction.
        let mut tx = self.pool.begin().await
            .map_err(|_| AppError::Internal)?;

        let chat_id = Uuid::new_v4();

        // Creating a chat by passing &mut *tx instead of a pool.
        sqlx::query!(
            "INSERT INTO chats (id, name, chat_type) VALUES ($1, $2, 'group')",
            chat_id,
            name,
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| AppError::Internal)?;

        // Add the creator as an administrator
        sqlx::query!(
            "INSERT INTO chat_members (chat_id, user_id, role) VALUES ($1, $2, 'admin')",
            chat_id,
            creator_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| AppError::Internal)?;

        // Adding other participants
        for member_id in initial_members {
            // Skip it if the creator accidentally passed their ID in the list
            if *member_id == creator_id { continue; }

            sqlx::query!(
                "INSERT INTO chat_members (chat_id, user_id, role) VALUES ($1, $2, 'member')",
                chat_id, member_id,
            )
            .execute(&mut *tx)
            .await?;
        }

        // Explicitly record the transaction
        tx.commit().await
            .map_err(|_| AppError::Internal)?;

        Ok(chat_id)
    }

    async fn get_or_create_private_chat(
        &self,
        user1_id: Uuid,
        user2_id: Uuid,
    ) -> Result<Uuid, AppError> {
        // Sorting the IDs for consistency
        let (user_a, user_b) = if user1_id < user2_id {
            (user1_id, user2_id)
        } 
        else {
            (user2_id, user1_id)
        };

        // Trying to find an existing chat (using JOIN to search for an intersection)
        let existing_chat = sqlx::query_scalar!(
            r#"
            SELECT c.id
            FROM chats c
            JOIN chat_members m1 ON c.id = m1.chat_id
            JOIN chat_members m2 ON c.id = m2.chat_id
            WHERE c.chat_type = 'private'
              AND m1.user_id = $1
              AND m2.user_id = $2
            "#,
            user_a, user_b,
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(chat_id) = existing_chat {
            return Ok(chat_id);
        }

        // If not found, create in the transaction
        let mut tx = self.pool.begin().await?;

        let new_chat_id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO chats (id, chat_type) VALUES ($1, 'private')",
            new_chat_id,
        )
        .execute(&mut *tx)
        .await?;

        // Adding both participants
        sqlx::query!(
            "INSERT INTO chat_members (chat_id, user_id, role) VALUES ($1, $2, 'member'),
            ($1, $3, 'member')",
            new_chat_id, user_a, user_b,
        )
        .execute(&mut *tx)
        .await?;

        // Fixing the transaction
        tx.commit().await?;

        Ok(new_chat_id)
    }
}
