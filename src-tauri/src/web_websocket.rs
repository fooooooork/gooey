use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::broadcast;
use tauri::AppHandle;

pub struct WebSocketManager {
    tx: broadcast::Sender<WebSocketMessage>,
}

#[derive(Clone, Debug)]
pub struct WebSocketMessage {
    pub event: String,
    pub data: serde_json::Value,
}

impl WebSocketManager {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self { tx }
    }
    
    pub fn subscribe(&self) -> broadcast::Receiver<WebSocketMessage> {
        self.tx.subscribe()
    }
    
    pub fn broadcast(&self, message: WebSocketMessage) {
        let _ = self.tx.send(message);
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(_app_handle): State<AppHandle>,
) -> Response {
    ws.on_upgrade(websocket_connection)
}

async fn websocket_connection(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    
    // 发送欢迎消息
    let welcome = json!({
        "event": "connected",
        "data": {
            "timestamp": chrono::Utc::now(),
            "message": "WebSocket connected successfully"
        }
    });
    
    if let Ok(msg) = serde_json::to_string(&welcome) {
        let _ = sender.send(Message::Text(msg)).await;
    }
    
    // 处理接收到的消息
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    // 处理客户端发送的消息
                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                        handle_client_message(data).await;
                    }
                }
                Message::Close(_) => {
                    println!("WebSocket connection closed");
                    break;
                }
                Message::Ping(payload) => {
                    if let Err(e) = sender.send(Message::Pong(payload)).await {
                        println!("Failed to send pong: {}", e);
                        break;
                    }
                }
                _ => {}
            }
        }
    }
}

async fn handle_client_message(data: serde_json::Value) {
    // 处理客户端发送的 WebSocket 消息
    println!("Received WebSocket message: {:?}", data);
    
    // 这里可以添加具体的消息处理逻辑
    // 例如：订阅特定事件、发送命令等
}
