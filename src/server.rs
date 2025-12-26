//! MCP Server Implementation
//!
//! Handles JSON-RPC requests for system monitoring operations.

use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::error;

use crate::system_monitor::SystemMonitor;
use crate::types::*;

/// MCP Server for system monitoring requests
///
/// Implements Model Context Protocol server that processes JSON-RPC requests
/// for system information and monitoring operations.
#[derive(Debug)]
pub struct MCPServer {
    /// Shared reference to system monitor
    system_monitor: Arc<RwLock<SystemMonitor>>,
}

impl MCPServer {
    /// Creates new MCP server instance
    pub fn new(system_monitor: Arc<RwLock<SystemMonitor>>) -> Self {
        Self { system_monitor }
    }

    /// Handles MCP request and returns response
    pub async fn handle_request(&self, request: MCPRequest) -> MCPResponse {
        let id = request.id.clone();
        
        use tracing::info;
        info!("🔍 MCP Handler - Method: {}, ID: {:?}", request.method, id);

        match request.method.as_str() {
            "initialize" => self.handle_initialize(id, request.params).await,
            "initialized" => {
                info!("✅ Received initialized notification");
                // Notifications don't have responses, but return a dummy for consistency
                self.create_success_response(id, serde_json::json!({}))
            },
            "tools/list" => self.handle_tools_list(id).await,
            "tools/call" => self.handle_tools_call(id, request.params).await,
            METHOD_GET_SYSTEM_INFO => self.handle_get_system_info(id).await,
            METHOD_GET_CPU_INFO => self.handle_get_cpu_info(id).await,
            METHOD_GET_MEMORY_INFO => self.handle_get_memory_info(id).await,
            METHOD_GET_DISK_INFO => self.handle_get_disk_info(id).await,
            METHOD_GET_NETWORK_INFO => self.handle_get_network_info(id).await,
            METHOD_GET_PROCESSES => self.handle_get_processes(id).await,
            METHOD_GET_PROCESS_BY_PID => self.handle_get_process_by_pid(id, request.params).await,
            METHOD_GET_SYSTEM_METRICS => self.handle_get_system_metrics(id).await,
            METHOD_START_MONITORING => self.handle_start_monitoring(id).await,
            METHOD_STOP_MONITORING => self.handle_stop_monitoring(id).await,
            _ => self.create_error_response(id, ERROR_METHOD_NOT_FOUND, "Method not found"),
        }
    }

    /// Handles initialize method (MCP spec requirement)
    async fn handle_initialize(&self, id: Option<String>, params: Value) -> MCPResponse {
        // Extract requested protocol version from params
        let requested_version = params
            .get("protocolVersion")
            .and_then(|v| v.as_str())
            .unwrap_or("2025-11-25");
        
        let result = serde_json::json!({
            "protocolVersion": requested_version,
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "mcp-system-monitor",
                "version": "0.1.0"
            }
        });
        self.create_success_response(id, result)
    }

    /// Handles tools/list method (MCP spec requirement)
    async fn handle_tools_list(&self, id: Option<String>) -> MCPResponse {
        let result = serde_json::json!({
            "tools": [
                {
                    "name": "get_system_info",
                    "description": "Get system information (hostname, OS, kernel version, uptime)",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                },
                {
                    "name": "get_cpu_info",
                    "description": "Get CPU information and usage statistics",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                },
                {
                    "name": "get_memory_info",
                    "description": "Get memory and swap usage information",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                },
                {
                    "name": "get_disk_info",
                    "description": "Get disk usage information for all mounted filesystems",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                },
                {
                    "name": "get_network_info",
                    "description": "Get network interface information and statistics",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                },
                {
                    "name": "get_processes",
                    "description": "Get list of all running processes",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                },
                {
                    "name": "get_system_metrics",
                    "description": "Get comprehensive system metrics",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                }
            ]
        });
        self.create_success_response(id, result)
    }

    /// Handles tools/call method (MCP spec requirement)
    async fn handle_tools_call(&self, id: Option<String>, params: Value) -> MCPResponse {
        let tool_name = params.get("name").and_then(|v| v.as_str());
        
        use tracing::info;
        if let Some(name) = tool_name {
            info!("🔧 Calling tool: {}", name);
        }
        
        let tool_response = match tool_name {
            Some("get_system_info") => self.handle_get_system_info(id.clone()).await,
            Some("get_cpu_info") => self.handle_get_cpu_info(id.clone()).await,
            Some("get_memory_info") => self.handle_get_memory_info(id.clone()).await,
            Some("get_disk_info") => self.handle_get_disk_info(id.clone()).await,
            Some("get_network_info") => self.handle_get_network_info(id.clone()).await,
            Some("get_processes") => self.handle_get_processes(id.clone()).await,
            Some("get_system_metrics") => self.handle_get_system_metrics(id.clone()).await,
            _ => return self.create_error_response(id, ERROR_METHOD_NOT_FOUND, "Tool not found"),
        };
        
        // Wrap result in MCP content format for tools/call responses
        match tool_response.result {
            Some(result) => {
                let content = vec![serde_json::json!({
                    "type": "text",
                    "text": serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
                })];
                self.create_success_response(id, serde_json::json!({ "content": content }))
            }
            None => tool_response
        }
    }

    /// Handles getSystemInfo method
    async fn handle_get_system_info(&self, id: Option<String>) -> MCPResponse {
        let mut monitor = self.system_monitor.write().await;
        match monitor.get_system_info() {
            Ok(info) => {
                let result = serde_json::to_value(info).unwrap_or_default();
                self.create_success_response(id, result)
            }
            Err(e) => {
                error!("Failed to get system info: {}", e);
                self.create_error_response(
                    id,
                    ERROR_INTERNAL_ERROR,
                    &format!("Failed to get system info: {}", e),
                )
            }
        }
    }

    /// Handles getCPUInfo method
    async fn handle_get_cpu_info(&self, id: Option<String>) -> MCPResponse {
        let mut monitor = self.system_monitor.write().await;
        match monitor.get_cpu_info() {
            Ok(info) => {
                let result = serde_json::to_value(info).unwrap_or_default();
                self.create_success_response(id, result)
            }
            Err(e) => {
                error!("Failed to get CPU info: {}", e);
                self.create_error_response(
                    id,
                    ERROR_INTERNAL_ERROR,
                    &format!("Failed to get CPU info: {}", e),
                )
            }
        }
    }

    /// Handles getMemoryInfo method
    async fn handle_get_memory_info(&self, id: Option<String>) -> MCPResponse {
        let mut monitor = self.system_monitor.write().await;
        match monitor.get_memory_info() {
            Ok(info) => {
                let result = serde_json::to_value(info).unwrap_or_default();
                self.create_success_response(id, result)
            }
            Err(e) => {
                error!("Failed to get memory info: {}", e);
                self.create_error_response(
                    id,
                    ERROR_INTERNAL_ERROR,
                    &format!("Failed to get memory info: {}", e),
                )
            }
        }
    }

    /// Handles getDiskInfo method
    async fn handle_get_disk_info(&self, id: Option<String>) -> MCPResponse {
        let mut monitor = self.system_monitor.write().await;
        match monitor.get_disk_info() {
            Ok(disks) => {
                let result = serde_json::to_value(disks).unwrap_or_default();
                self.create_success_response(id, result)
            }
            Err(e) => {
                error!("Failed to get disk info: {}", e);
                self.create_error_response(
                    id,
                    ERROR_INTERNAL_ERROR,
                    &format!("Failed to get disk info: {}", e),
                )
            }
        }
    }

    /// Handles getNetworkInfo method
    async fn handle_get_network_info(&self, id: Option<String>) -> MCPResponse {
        let mut monitor = self.system_monitor.write().await;
        match monitor.get_network_info() {
            Ok(networks) => {
                let result = serde_json::to_value(networks).unwrap_or_default();
                self.create_success_response(id, result)
            }
            Err(e) => {
                error!("Failed to get network info: {}", e);
                self.create_error_response(
                    id,
                    ERROR_INTERNAL_ERROR,
                    &format!("Failed to get network info: {}", e),
                )
            }
        }
    }

    /// Handles getProcesses method
    async fn handle_get_processes(&self, id: Option<String>) -> MCPResponse {
        let mut monitor = self.system_monitor.write().await;
        match monitor.get_processes() {
            Ok(processes) => {
                let result = serde_json::to_value(processes).unwrap_or_default();
                self.create_success_response(id, result)
            }
            Err(e) => {
                error!("Failed to get processes: {}", e);
                self.create_error_response(
                    id,
                    ERROR_INTERNAL_ERROR,
                    &format!("Failed to get processes: {}", e),
                )
            }
        }
    }

    /// Handles getProcessByPID method
    async fn handle_get_process_by_pid(&self, id: Option<String>, params: Value) -> MCPResponse {
        let pid = match params.get("pid") {
            Some(pid_value) => match pid_value.as_u64() {
                Some(pid) => pid as u32,
                None => {
                    return self.create_error_response(
                        id,
                        ERROR_INVALID_PARAMS,
                        "Invalid PID parameter",
                    );
                }
            },
            None => {
                return self.create_error_response(
                    id,
                    ERROR_INVALID_PARAMS,
                    "Missing PID parameter",
                );
            }
        };

        let mut monitor = self.system_monitor.write().await;
        match monitor.get_process_by_pid(pid) {
            Ok(Some(process)) => {
                let result = serde_json::to_value(process).unwrap_or_default();
                self.create_success_response(id, result)
            }
            Ok(None) => self.create_error_response(
                id,
                ERROR_PROCESS_NOT_FOUND,
                &format!("Process with PID {} not found", pid),
            ),
            Err(e) => {
                error!("Failed to get process by PID {}: {}", pid, e);
                self.create_error_response(
                    id,
                    ERROR_INTERNAL_ERROR,
                    &format!("Failed to get process: {}", e),
                )
            }
        }
    }

    /// Handles getSystemMetrics method
    async fn handle_get_system_metrics(&self, id: Option<String>) -> MCPResponse {
        let mut monitor = self.system_monitor.write().await;
        match monitor.get_system_metrics() {
            Ok(metrics) => {
                let result = serde_json::to_value(metrics).unwrap_or_default();
                self.create_success_response(id, result)
            }
            Err(e) => {
                error!("Failed to get system metrics: {}", e);
                self.create_error_response(
                    id,
                    ERROR_INTERNAL_ERROR,
                    &format!("Failed to get system metrics: {}", e),
                )
            }
        }
    }

    /// Handles startMonitoring method
    async fn handle_start_monitoring(&self, id: Option<String>) -> MCPResponse {
        let mut monitor = self.system_monitor.write().await;
        match monitor.start_monitoring() {
            Ok(started) => {
                let result = serde_json::json!({
                    "started": started,
                    "message": if started { "Monitoring started successfully" } else { "Monitoring already active" }
                });
                self.create_success_response(id, result)
            }
            Err(e) => {
                error!("Failed to start monitoring: {}", e);
                self.create_error_response(
                    id,
                    ERROR_INTERNAL_ERROR,
                    &format!("Failed to start monitoring: {}", e),
                )
            }
        }
    }

    /// Handles stopMonitoring method
    async fn handle_stop_monitoring(&self, id: Option<String>) -> MCPResponse {
        let mut monitor = self.system_monitor.write().await;
        match monitor.stop_monitoring() {
            Ok(stopped) => {
                let result = serde_json::json!({
                    "stopped": stopped,
                    "message": if stopped { "Monitoring stopped successfully" } else { "Monitoring not active" }
                });
                self.create_success_response(id, result)
            }
            Err(e) => {
                error!("Failed to stop monitoring: {}", e);
                self.create_error_response(
                    id,
                    ERROR_INTERNAL_ERROR,
                    &format!("Failed to stop monitoring: {}", e),
                )
            }
        }
    }

    /// Creates successful MCP response
    fn create_success_response(&self, id: Option<String>, result: Value) -> MCPResponse {
        MCPResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Creates error MCP response
    fn create_error_response(&self, id: Option<String>, code: i32, message: &str) -> MCPResponse {
        MCPResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(MCPError {
                code,
                message: message.to_string(),
                data: None,
            }),
        }
    }
}
