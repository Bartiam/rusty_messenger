use axum::{
    Router, 
    middleware, 
    routing::post
};

use crate::{
    api::handlers, 
    middleware::auth::auth_middleware, 
    state::AppState
};


pub fn app_router(state: AppState) -> Router {
    // Create a router with private routes.
    let private_routes = Router::new()
        .route("/chats", post(handlers::create_chat_handler))
        .route("/messages", post(handlers::send_message_handler))
        // Protecting ALL routes above this line
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware
        ));
    
    // Create a router with public routes.
    let public_routes = Router::new()
        .route("/auth/register", post(handlers::register_handler))
        .route("/auth/login", post(handlers::login_handler));

    // Combining them
    Router::new()
        .merge(private_routes)
        .merge(public_routes)
        .with_state(state)
}