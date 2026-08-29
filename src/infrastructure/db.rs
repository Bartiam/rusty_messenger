use redis::{
    Client, 
    aio::MultiplexedConnection
};

pub mod user_repo;
pub mod chat;
pub mod message_repo;

pub async fn connect_redis(redis_url: &str) -> Result<MultiplexedConnection, redis::RedisError> {
    let client = Client::open(redis_url)?;

    // Create a multiplexed connection to separate tasks.
    let con = client
        .get_multiplexed_async_connection()
        .await?;

    Ok(con)
}