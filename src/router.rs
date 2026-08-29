use axum::{
    Router, 
    middleware, 
    routing::{
        delete, 
        get, 
        post
    }
};

use crate::{
    api::handlers::{
        auth::login_user_handler, 
        chats::{
            create_group_chat, 
            create_private_chat
        }, 
        health_check, 
        messages::{
            get_messages_handler, 
            send_message_handler, 
            delete_message_handler, 
            admin_get_user_messages
        }, 
        users::create_user_handler
    }, 
    middleware::auth::auth_middleware, 
    state::AppState
};


pub fn app_router(state: AppState) -> Router {
    let private_routes = Router::new()
        .route("/chats", post(create_private_chat))
        .route("/chats/group", post(create_group_chat))
        .route("/chats/{chat_id}/messages", post(send_message_handler))
        .route("/chats/{chat_id}/messages", get(get_messages_handler))
        .route("/messages/{message_id}", delete(delete_message_handler))
        .route("/admin/users/{user_id}/messages", get(admin_get_user_messages))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", post(login_user_handler))
        .route("/auth/register", post(create_user_handler));

    Router::new()
        .merge(private_routes)
        .merge(public_routes)
        .with_state(state)
}