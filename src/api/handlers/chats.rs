use std::sync::Arc;

use axum::{
    Extension, 
    Json, 
    extract::State, 
    http::StatusCode, 
    response::IntoResponse
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    domain::repositories::ChatRepository, 
    error::AppError, 
    middleware::auth::CurrentUser
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePrivateChatReq {
    pub target_user_id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGroupChatReq {
    #[validate(length(min = 3, max = 100, message = "Title must be between 3 and 100 characters."))]
    pub title: String,
    #[validate(length(min = 1, message = "At least one member os required"))]
    pub members: Vec<Uuid>,
}

pub async fn create_private_chat(
    State(repo): State<Arc<dyn ChatRepository>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(payload): Json<CreatePrivateChatReq>,
) -> Result<impl IntoResponse, AppError>{
    // Protection against creating a chat with oneself
    if current_user.id == payload.target_user_id {
        return Err(AppError::InvalidInput("Cannot create chat with yourself".into()));
    }

    let chat_id = repo
        .get_or_create_private_chat(current_user.id, payload.target_user_id)
        .await?;

    // Return 201 Created and JSON with the chat ID
    Ok((StatusCode::CREATED, Json(serde_json::json!({ "chat_id": chat_id }))))
}

pub async fn create_group_chat(
    State(repo): State<Arc<dyn ChatRepository>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(payload): Json<CreateGroupChatReq>,
) -> Result<impl IntoResponse, AppError> {
    // Base validate
    if payload.title.trim().is_empty() {
        return Err(AppError::InvalidInput("Title cannot be empty".into()));
    }

    let chat_id = repo
        .create_group_chat(current_user.id, &payload.title, &payload.members)
        .await?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({ "chat_id": chat_id }))))
}
