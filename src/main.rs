use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/ws", get(websocket_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();
    tracing::info!("Сервер запущен на http://localhost:8000 и ws://localhost:8000/ws");
    axum::serve(listener, app).await.unwrap();


}

async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    tracing::info!("Клиент подключился!");

    while let Some(Ok(msg)) = socket.recv().await {
        tracing::info!("Получено сообщение: {:?}", msg);

        if socket.send(msg).await.is_err() {
            tracing::warn!("Клиент отключился при отправке!");
            break;
        }
    }

    tracing::info!("Клиент отключился!");
}