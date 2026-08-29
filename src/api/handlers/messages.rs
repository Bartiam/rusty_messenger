use axum::{
    Extension, 
    Json, 
    extract::{
        Path, 
        Query, 
        State
    }, 
    http::StatusCode, 
    response::IntoResponse
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    domain::models::{
        Message, 
        SendMessageRequest
    }, 
    error::AppError, 
    middleware::auth::CurrentUser, 
    state::AppState, 
    validation::validate_input
};


// --- Pagination parameters ---
#[derive(Debug, Deserialize)]
pub struct GetMessagesQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn send_message_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(chat_id): Path<Uuid>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Content validation
    validate_input(&payload)?;

    // Checking the chat membership
    let is_member = state.message_repo.is_user_in_chat(current_user.id, chat_id).await?;
    if !is_member {
        return Err(AppError::InvalidInput("You are not a member of this chat.".to_string()));
    }

    let message = state
        .message_repo
        .send_message(
            chat_id, 
            current_user.id, 
            &payload.content)
        .await?;

    Ok((StatusCode::CREATED, Json(message)))

}

// --- Retrieving the history (active messages only) ---
pub async fn get_messages_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(chat_id): Path<Uuid>,
    Query(query): Query<GetMessagesQuery>,
) -> Result<Json<Vec<Message>>, AppError> {
    let is_member = state.message_repo.is_user_in_chat(current_user.id, chat_id).await?;
    if !is_member {
        return Err(AppError::InvalidInput("You are not a member of this chat.".to_string()));
    }

    let limit = query.limit.unwrap_or(50).clamp(1, 100);
    let offset = query.offset.unwrap_or(0).max(0);

    // include_deleted = false - show only active items
    let messages = state
        .message_repo
        .get_messages(
            chat_id, 
            limit, 
            offset, 
            false)
        .await?;

    Ok(Json(messages))
}

// --- Soft deletion of a message (author or admin) ---
pub async fn delete_message_handler(
    State(state): State<AppState>,
    Extension(current_user): Extension<CurrentUser>,
    Path(message_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Receive a message to verify the rights.
    let message = sqlx::query_as!(
        Message,
        "SELECT id, chat_id, sender_id, content, created_at, updated_at, deleted_at FROM messages WHERE id = $1",
        message_id,
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    // Check that the message has not yet been deleted.
    if message.deleted_at.is_some() {
        return Err(AppError::Unauthorized);
    }

    // Only the author (or the chat admin - omitted here) is allowed to delete.
    if message.sender_id != Some(current_user.id) {
        return  Err(AppError::Unauthorized);
    }

    state.message_repo.delete_message(message_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

// --- Administrative access to the user's full history ---
pub async fn admin_get_user_messages(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<Message>>, AppError> {
    let messages = state.message_repo.get_user_messages_all(user_id).await?;

    Ok(Json(messages))
}
