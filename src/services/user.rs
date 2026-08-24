use redis::{AsyncCommands, aio::MultiplexedConnection};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::models::UserProfile, error::AppError};


pub async fn get_user_profile(
    redis: &mut MultiplexedConnection,
    pg_pool: &PgPool,
    user_id: Uuid,
) -> Result<UserProfile, AppError> {
    let cache_key = format!("user:{}", user_id);

    // Trying to get it from Redis
    let cached: Option<String> = redis
        .get(&cache_key)
        .await
        .unwrap_or(None);

    if let Some(json_str) = cached {
        // Cache Hit: deserialize and return
        if let Ok(profile) = serde_json::from_str(&json_str) {
            return Ok(profile);
        }
    }

    // Cache Miss. Going to PostgreSQL
    let profile = sqlx::query_as!(
        UserProfile,
        "SELECT id, username, bio FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(pg_pool)
    .await?;

    // Save to cache for 5 minutes (300 seconds).
    if let Ok(json_str) = serde_json::to_string(&profile) {
        let _: Result<(), _> = redis.set_ex(&cache_key, json_str, 300)
            .await;
    }

    Ok(profile)
}
