use axum::{
    Extension, 
    Json, 
    response::IntoResponse
};

use serde_json::json;

use crate::middleware::auth::CurrentUser;

// PLUG FUNCTION //
pub async fn send_message_handler(
    Extension(current_user): Extension<CurrentUser>,
) -> impl IntoResponse {
    Json(json!({
        "message": "Message sent",
        "user_id": current_user.id
    }))
}