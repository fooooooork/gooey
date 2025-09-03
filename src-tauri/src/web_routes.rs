use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post, put, delete},
};
use serde_json::Value;
use tauri::AppHandle;

// 项目相关路由
pub async fn list_projects_web(
    State(_app_handle): State<AppHandle>,
) -> Result<Json<Vec<crate::commands::claude::Project>>, StatusCode> {
    match crate::commands::claude::list_projects().await {
        Ok(projects) => Ok(Json(projects)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_project_web(
    State(_app_handle): State<AppHandle>,
    Json(payload): Json<Value>,
) -> Result<Json<crate::commands::claude::Project>, StatusCode> {
    let path = payload["path"].as_str().unwrap_or("");
    match crate::commands::claude::create_project(path.to_string()).await {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_project_web(
    State(_app_handle): State<AppHandle>,
    Path(id): Path<String>,
) -> Result<Json<crate::commands::claude::Project>, StatusCode> {
    // 实现获取单个项目的逻辑
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_project_sessions_web(
    State(_app_handle): State<AppHandle>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<crate::commands::claude::Session>>, StatusCode> {
    match crate::commands::claude::get_project_sessions(project_id).await {
        Ok(sessions) => Ok(Json(sessions)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 代理相关路由
pub async fn list_agents_web(
    State(_app_handle): State<AppHandle>,
) -> Result<Json<Vec<crate::commands::agents::Agent>>, StatusCode> {
    match crate::commands::agents::list_agents().await {
        Ok(agents) => Ok(Json(agents)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_agent_web(
    State(_app_handle): State<AppHandle>,
    Json(payload): Json<Value>,
) -> Result<Json<crate::commands::agents::Agent>, StatusCode> {
    let name = payload["name"].as_str().unwrap_or("");
    let icon = payload["icon"].as_str().unwrap_or("");
    let system_prompt = payload["system_prompt"].as_str().unwrap_or("");
    let default_task = payload["default_task"].as_str();
    let model = payload["model"].as_str().unwrap_or("sonnet");
    
    match crate::commands::agents::create_agent(
        name.to_string(),
        icon.to_string(),
        system_prompt.to_string(),
        default_task.map(|s| s.to_string()),
        Some(model.to_string()),
        None,
    ).await {
        Ok(agent) => Ok(Json(agent)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_agent_web(
    State(_app_handle): State<AppHandle>,
    Path(id): Path<u32>,
) -> Result<Json<crate::commands::agents::Agent>, StatusCode> {
    match crate::commands::agents::get_agent(id).await {
        Ok(agent) => Ok(Json(agent)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_agent_web(
    State(_app_handle): State<AppHandle>,
    Path(id): Path<u32>,
    Json(payload): Json<Value>,
) -> Result<Json<crate::commands::agents::Agent>, StatusCode> {
    let name = payload["name"].as_str().unwrap_or("");
    let icon = payload["icon"].as_str().unwrap_or("");
    let system_prompt = payload["system_prompt"].as_str().unwrap_or("");
    let default_task = payload["default_task"].as_str();
    let model = payload["model"].as_str().unwrap_or("sonnet");
    
    match crate::commands::agents::update_agent(
        id,
        name.to_string(),
        icon.to_string(),
        system_prompt.to_string(),
        default_task.map(|s| s.to_string()),
        Some(model.to_string()),
        None,
    ).await {
        Ok(agent) => Ok(Json(agent)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_agent_web(
    State(_app_handle): State<AppHandle>,
    Path(id): Path<u32>,
) -> Result<Json<Value>, StatusCode> {
    match crate::commands::agents::delete_agent(id).await {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn execute_agent_web(
    State(_app_handle): State<AppHandle>,
    Path(id): Path<u32>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let project_path = payload["project_path"].as_str().unwrap_or("");
    let task = payload["task"].as_str().unwrap_or("");
    let model = payload["model"].as_str();
    
    match crate::commands::agents::execute_agent(
        id,
        project_path.to_string(),
        task.to_string(),
        model.map(|s| s.to_string()),
    ).await {
        Ok(run_id) => Ok(Json(serde_json::json!({ "run_id": run_id }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 会话相关路由
pub async fn list_sessions_web(
    State(_app_handle): State<AppHandle>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    // 实现会话列表逻辑
    Ok(Json(vec![]))
}

pub async fn get_session_web(
    State(_app_handle): State<AppHandle>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    // 实现获取会话逻辑
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_session_output_web(
    State(_app_handle): State<AppHandle>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    // 实现获取会话输出逻辑
    Ok(Json(serde_json::json!({ "output": "" })))
}

pub async fn execute_claude_code_web(
    State(_app_handle): State<AppHandle>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // 实现执行 Claude 代码逻辑
    Ok(Json(serde_json::json!({ "success": true })))
}

// 使用统计路由
pub async fn get_usage_stats_web(
    State(_app_handle): State<AppHandle>,
) -> Result<Json<Value>, StatusCode> {
    match crate::commands::usage::get_usage_stats().await {
        Ok(stats) => Ok(Json(serde_json::to_value(stats).unwrap_or(serde_json::json!({})))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_usage_by_date_range_web(
    State(_app_handle): State<AppHandle>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let start_date = payload["start_date"].as_str().unwrap_or("");
    let end_date = payload["end_date"].as_str().unwrap_or("");
    
    match crate::commands::usage::get_usage_by_date_range(start_date.to_string(), end_date.to_string()).await {
        Ok(stats) => Ok(Json(serde_json::to_value(stats).unwrap_or(serde_json::json!({})))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// MCP 服务器路由
pub async fn mcp_list_web(
    State(_app_handle): State<AppHandle>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    match crate::commands::mcp::mcp_list().await {
        Ok(servers) => Ok(Json(serde_json::to_value(servers).unwrap_or(vec![]))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn mcp_add_web(
    State(_app_handle): State<AppHandle>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // 实现添加 MCP 服务器逻辑
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn mcp_remove_web(
    State(_app_handle): State<AppHandle>,
    Path(name): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    match crate::commands::mcp::mcp_remove(name).await {
        Ok(_) => Ok(Json(serde_json::json!({ "success": true }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn mcp_test_connection_web(
    State(_app_handle): State<AppHandle>,
    Path(name): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    match crate::commands::mcp::mcp_test_connection(name).await {
        Ok(result) => Ok(Json(serde_json::json!({ "result": result }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// 检查点路由
pub async fn list_checkpoints_web(
    State(_app_handle): State<AppHandle>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    // 实现检查点列表逻辑
    Ok(Json(vec![]))
}

pub async fn create_checkpoint_web(
    State(_app_handle): State<AppHandle>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // 实现创建检查点逻辑
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn restore_checkpoint_web(
    State(_app_handle): State<AppHandle>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // 实现恢复检查点逻辑
    Ok(Json(serde_json::json!({ "success": true })))
}
