use axum::{
    response::Json,
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;
use serde_json::json;

pub async fn start_simple() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/projects", get(list_projects_simple))
        .route("/api/agents", get(list_agents_simple))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🌐 Simple Web server running on http://0.0.0.0:3000");
    println!("📱 Open http://localhost:3000 in your browser to test");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({ 
        "status": "ok", 
        "timestamp": chrono::Utc::now(),
        "version": "0.1.0"
    }))
}

async fn list_projects_simple() -> Json<Vec<serde_json::Value>> {
    Json(vec![
        json!({
            "id": "demo-project-1",
            "path": "/demo/project1",
            "sessions": [],
            "created_at": chrono::Utc::now().timestamp()
        })
    ])
}

async fn list_agents_simple() -> Json<Vec<serde_json::Value>> {
    Json(vec![
        json!({
            "id": 1,
            "name": "Demo Agent",
            "icon": "bot",
            "system_prompt": "You are a helpful assistant",
            "model": "sonnet",
            "created_at": chrono::Utc::now().to_rfc3339(),
            "updated_at": chrono::Utc::now().to_rfc3339()
        })
    ])
}
