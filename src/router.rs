use axum::{
    Router, 
    middleware, 
    routing::{
        get, 
        post
    }
};

use crate::{
    api::handlers::{
        auth::{
            login_user_handler, 
            register_handler
        }, 
        chats::{
            create_group_chat, 
            create_private_chat
        }, 
        health_check, 
        messages::send_message_handler
    }, 
    middleware::auth::auth_middleware, state::AppState
};


pub fn app_router(state: AppState) -> Router {
    // Create a router with private routes.
    let private_routes = Router::new()
        .route("/chats", post(create_private_chat))
        .route("/chats/group", post(create_group_chat))
        .route("/messages", post(send_message_handler))
        // Protecting ALL routes above this line
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware
        ));
    
    // Create a router with public routes.
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_user_handler));

    // Combining them
    Router::new()
        .merge(private_routes)
        .merge(public_routes)
        .with_state(state)
}