//! HTTP Server Implementation
//!
//! REST API server using Axum for system monitoring data access.

use axum::{
    extract::State,
    http::StatusCode,
    response::{sse::Event, Sse, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use futures::stream;
use futures::stream::StreamExt;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::server::MCPServer;
use crate::system_monitor::SystemMonitor;
use crate::types::*;
use axum::extract::Request;
use axum::middleware::Next;

async fn log_requests(req: Request, next: Next) -> impl IntoResponse {
    let timestamp = chrono::Utc::now().format("%H:%M:%S%.3f");
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let headers = req.headers();
    let accept = headers.get("accept").map(|h| h.to_str().unwrap_or("?")).unwrap_or("none");
    let content_type = headers.get("content-type").map(|h| h.to_str().unwrap_or("?")).unwrap_or("none");
    
    info!("[{}] {} {} (Accept: {}, Content-Type: {})", timestamp, method, path, accept, content_type);
    
    if path == "/" && accept.contains("text/event-stream") {
        info!("🔍 SSE CONNECTION DETECTED - Full headers:");
        for (name, value) in headers.iter() {
            let value_str = value.to_str().unwrap_or("<binary>");
            info!("  {}: {}", name, value_str);
        }
    }
    
    let res = next.run(req).await;
    info!("  → {} {}", res.status().as_u16(), res.status().canonical_reason().unwrap_or("?"));
    res
}

/// HTTP Server for REST API access to system monitoring
///
/// Implements REST API server using Axum that provides HTTP endpoints
/// for accessing system monitoring data.
#[derive(Debug)]
pub struct HTTPServer {
    /// Internal MCP server for system monitoring requests
    mcp_server: MCPServer,
}

impl HTTPServer {
    /// Creates new HTTP server instance
    pub fn new(system_monitor: Arc<RwLock<SystemMonitor>>) -> Self {
        Self {
            mcp_server: MCPServer::new(system_monitor),
        }
    }

    /// Runs HTTP server on specified port
    pub async fn run(self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let mcp_server = Arc::new(self.mcp_server);

        let app = Router::new()
            .route("/", post(Self::handle_mcp_request).get(Self::handle_sse))
            .route("/health", get(Self::health_check))
            .layer(axum::middleware::from_fn(log_requests))
            .route("/api/system/info", get(Self::get_system_info))
            .route("/api/system/cpu", get(Self::get_cpu_info))
            .route("/api/system/memory", get(Self::get_memory_info))
            .route("/api/system/disks", get(Self::get_disk_info))
            .route("/api/system/networks", get(Self::get_network_info))
            .route("/api/system/processes", get(Self::get_processes))
            .route("/api/system/processes/:pid", get(Self::get_process_by_pid))
            .route("/api/system/metrics", get(Self::get_system_metrics))
            .route("/api/monitoring/start", post(Self::start_monitoring))
            .route("/api/monitoring/stop", post(Self::stop_monitoring))
            .route("/api/monitoring/status", get(Self::get_monitoring_status))
            .with_state(mcp_server);

        let addr = format!("0.0.0.0:{}", port);
        info!("Starting HTTP server on {}", addr);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }

    /// Health check endpoint
    async fn health_check() -> Json<Value> {
        info!("🏥 Health check requested");
        Json(json!({
            "status": "healthy",
            "service": "MCP System Monitor",
            "timestamp": chrono::Utc::now()
        }))
    }

    /// GET /api/system/info - Get system information
    async fn get_system_info(
        State(server): State<Arc<MCPServer>>,
    ) -> Result<Json<Value>, StatusCode> {
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(uuid::Uuid::new_v4().to_string()),
            method: METHOD_GET_SYSTEM_INFO.to_string(),
            params: json!({}),
        };

        let response = server.handle_request(request).await;
        match response.result {
            Some(result) => Ok(Json(result)),
            None => {
                error!("Failed to get system info: {:?}", response.error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// GET /api/system/cpu - Get CPU information
    async fn get_cpu_info(State(server): State<Arc<MCPServer>>) -> Result<Json<Value>, StatusCode> {
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(uuid::Uuid::new_v4().to_string()),
            method: METHOD_GET_CPU_INFO.to_string(),
            params: json!({}),
        };

        let response = server.handle_request(request).await;
        match response.result {
            Some(result) => Ok(Json(result)),
            None => {
                error!("Failed to get CPU info: {:?}", response.error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// GET /api/system/memory - Get memory information
    async fn get_memory_info(
        State(server): State<Arc<MCPServer>>,
    ) -> Result<Json<Value>, StatusCode> {
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(uuid::Uuid::new_v4().to_string()),
            method: METHOD_GET_MEMORY_INFO.to_string(),
            params: json!({}),
        };

        let response = server.handle_request(request).await;
        match response.result {
            Some(result) => Ok(Json(result)),
            None => {
                error!("Failed to get memory info: {:?}", response.error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// GET /api/system/disks - Get disk information
    async fn get_disk_info(
        State(server): State<Arc<MCPServer>>,
    ) -> Result<Json<Value>, StatusCode> {
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(uuid::Uuid::new_v4().to_string()),
            method: METHOD_GET_DISK_INFO.to_string(),
            params: json!({}),
        };

        let response = server.handle_request(request).await;
        match response.result {
            Some(result) => Ok(Json(result)),
            None => {
                error!("Failed to get disk info: {:?}", response.error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// GET /api/system/networks - Get network information
    async fn get_network_info(
        State(server): State<Arc<MCPServer>>,
    ) -> Result<Json<Value>, StatusCode> {
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(uuid::Uuid::new_v4().to_string()),
            method: METHOD_GET_NETWORK_INFO.to_string(),
            params: json!({}),
        };

        let response = server.handle_request(request).await;
        match response.result {
            Some(result) => Ok(Json(result)),
            None => {
                error!("Failed to get network info: {:?}", response.error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// GET /api/system/processes - Get all processes
    async fn get_processes(
        State(server): State<Arc<MCPServer>>,
    ) -> Result<Json<Value>, StatusCode> {
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(uuid::Uuid::new_v4().to_string()),
            method: METHOD_GET_PROCESSES.to_string(),
            params: json!({}),
        };

        let response = server.handle_request(request).await;
        match response.result {
            Some(result) => Ok(Json(result)),
            None => {
                error!("Failed to get processes: {:?}", response.error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// GET /api/system/processes/{pid} - Get specific process by PID
    async fn get_process_by_pid(
        State(server): State<Arc<MCPServer>>,
        axum::extract::Path(pid): axum::extract::Path<u32>,
    ) -> Result<Json<Value>, StatusCode> {
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(uuid::Uuid::new_v4().to_string()),
            method: METHOD_GET_PROCESS_BY_PID.to_string(),
            params: json!({"pid": pid}),
        };

        let response = server.handle_request(request).await;
        match response.result {
            Some(result) => Ok(Json(result)),
            None => {
                error!("Failed to get process by PID {}: {:?}", pid, response.error);
                Err(StatusCode::NOT_FOUND)
            }
        }
    }

    /// GET /api/system/metrics - Get complete system metrics
    async fn get_system_metrics(
        State(server): State<Arc<MCPServer>>,
    ) -> Result<Json<Value>, StatusCode> {
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(uuid::Uuid::new_v4().to_string()),
            method: METHOD_GET_SYSTEM_METRICS.to_string(),
            params: json!({}),
        };

        let response = server.handle_request(request).await;
        match response.result {
            Some(result) => Ok(Json(result)),
            None => {
                error!("Failed to get system metrics: {:?}", response.error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// POST /api/monitoring/start - Start continuous monitoring
    async fn start_monitoring(
        State(server): State<Arc<MCPServer>>,
    ) -> Result<Json<Value>, StatusCode> {
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(uuid::Uuid::new_v4().to_string()),
            method: METHOD_START_MONITORING.to_string(),
            params: json!({}),
        };

        let response = server.handle_request(request).await;
        match response.result {
            Some(result) => Ok(Json(result)),
            None => {
                error!("Failed to start monitoring: {:?}", response.error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// POST /api/monitoring/stop - Stop continuous monitoring
    async fn stop_monitoring(
        State(server): State<Arc<MCPServer>>,
    ) -> Result<Json<Value>, StatusCode> {
        let request = MCPRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(uuid::Uuid::new_v4().to_string()),
            method: METHOD_STOP_MONITORING.to_string(),
            params: json!({}),
        };

        let response = server.handle_request(request).await;
        match response.result {
            Some(result) => Ok(Json(result)),
            None => {
                error!("Failed to stop monitoring: {:?}", response.error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// GET /api/monitoring/status - Get monitoring status
    async fn get_monitoring_status(State(_server): State<Arc<MCPServer>>) -> Json<Value> {
        // This endpoint provides monitoring status information
        // For now, we'll return a simple status response
        Json(json!({
            "monitoring_active": true, // This would need to be implemented
            "last_update": chrono::Utc::now(),
            "service_status": "running"
        }))
    }

    /// POST / - Handle MCP JSON-RPC requests (return JSON, not SSE)
    async fn handle_mcp_request(
        State(server): State<Arc<MCPServer>>,
        body: axum::body::Body,
    ) -> Result<Json<Value>, String> {
        // Read the entire body
        let bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .map_err(|e| format!("Failed to read body: {}", e))?;
        
        // Parse JSON
        let request: MCPRequest = serde_json::from_slice(&bytes)
            .map_err(|e| {
                let body_str = String::from_utf8_lossy(&bytes);
                error!("❌ JSON parse error: {}", e);
                error!("   Raw body ({} bytes): {}", bytes.len(), body_str);
                format!("JSON parse error: {}", e)
            })?;
        
        let timestamp = chrono::Utc::now().format("%H:%M:%S%.3f");
        info!("[{}] 📨 POST / - MCP Request", timestamp);
        info!("  Method: {}", request.method);
        info!("  ID: {:?}", request.id);
        info!("  Params keys: {:?}", request.params.as_object().map(|o| o.keys().collect::<Vec<_>>()));
        info!("  Full request: {}", serde_json::to_string(&request).unwrap_or_default());
        
        // Check if this is a notification (no id field)
        let is_notification = request.id.is_none();
        
        let response = server.handle_request(request).await;
        
        info!("✅ MCP Response generated");
        info!("  ID: {:?}", response.id);
        info!("  Has result: {}", response.result.is_some());
        info!("  Has error: {}", response.error.is_some());
        
        // For notifications, don't send a response back
        if is_notification {
            info!("📤 Notification processed (no response)");
            return Ok(Json(json!({})));
        }
        
        info!("📤 Returning JSON response");
        
        // Convert MCPResponse to Value
        let response_value = serde_json::to_value(response)
            .unwrap_or_else(|_| json!({"error":"Failed to serialize"}));
        
        Ok(Json(response_value))
    }

    /// GET / - SSE stream for MCP clients
    async fn handle_sse(
    ) -> Sse<impl futures::Stream<Item = Result<Event, axum::Error>>> {
        let timestamp = chrono::Utc::now().format("%H:%M:%S%.3f");
        info!("[{}] 🔌 GET / - SSE Connection opened", timestamp);
        info!("🔧 Creating SSE stream with 10-second heartbeat interval");
        
        // Create a stream that sends heartbeat events every 10 seconds
        let stream = IntervalStream::new(interval(std::time::Duration::from_secs(10)))
            .enumerate()
            .map(|(idx, _)| {
                if idx == 0 {
                    info!("💓 Sending first keep-alive message");
                    // Send an empty message to signal connection is alive
                    Ok::<Event, axum::Error>(Event::default().data(""))
                } else {
                    info!("💓 Sending heartbeat #{}", idx);
                    // Send heartbeat comments for subsequent intervals
                    Ok::<Event, axum::Error>(Event::default().comment("heartbeat"))
                }
            });
         
        info!("🚀 SSE stream created and ready");
        Sse::new(stream)
    }

}
