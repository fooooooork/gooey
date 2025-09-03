use axum::{
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::CorsLayer;
use tauri::AppHandle;
use serde_json::json;

use crate::web_routes::*;

pub async fn start(app_handle: AppHandle) {
    let app = Router::new()
        // 项目相关 API
        .route("/api/projects", get(list_projects_web))
        .route("/api/projects", post(create_project_web))
        .route("/api/projects/:id", get(get_project_web))
        .route("/api/projects/:id/sessions", get(get_project_sessions_web))
        
        // 代理相关 API
        .route("/api/agents", get(list_agents_web))
        .route("/api/agents", post(create_agent_web))
        .route("/api/agents/:id", get(get_agent_web))
        .route("/api/agents/:id", put(update_agent_web))
        .route("/api/agents/:id", delete(delete_agent_web))
        .route("/api/agents/:id/execute", post(execute_agent_web))
        
        // 会话相关 API
        .route("/api/sessions", get(list_sessions_web))
        .route("/api/sessions/:id", get(get_session_web))
        .route("/api/sessions/:id/output", get(get_session_output_web))
        .route("/api/sessions/:id/execute", post(execute_claude_code_web))
        
        // 使用统计 API
        .route("/api/usage/stats", get(get_usage_stats_web))
        .route("/api/usage/by-date", get(get_usage_by_date_range_web))
        
        // MCP 服务器 API
        .route("/api/mcp/servers", get(mcp_list_web))
        .route("/api/mcp/servers", post(mcp_add_web))
        .route("/api/mcp/servers/:name", delete(mcp_remove_web))
        .route("/api/mcp/servers/:name/test", post(mcp_test_connection_web))
        
        // 检查点 API
        .route("/api/checkpoints", get(list_checkpoints_web))
        .route("/api/checkpoints", post(create_checkpoint_web))
        .route("/api/checkpoints/:id/restore", post(restore_checkpoint_web))
        
        // WebSocket 连接
        .route("/ws", get(crate::web_websocket::websocket_handler))
        
        // 健康检查
        .route("/health", get(health_check))
        
        .layer(CorsLayer::permissive())
        .with_state(app_handle);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🌐 Web server running on http://0.0.0.0:3000");
    println!("📱 Open http://localhost:3000 in your browser to use the web version");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({ 
        "status": "ok", 
        "timestamp": chrono::Utc::now(),
        "version": "0.1.0"
    }))
}
