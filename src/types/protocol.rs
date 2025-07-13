//! MCP Protocol Types
//!
//! Core JSON-RPC protocol structures for MCP communication.

use serde::{Deserialize, Serialize};

/// MCP Request structure following JSON-RPC 2.0 specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRequest {
    /// JSON-RPC version (should be "2.0")
    pub jsonrpc: String,
    /// Unique request identifier
    pub id: String,
    /// Method name to be invoked
    pub method: String,
    /// Method parameters (JSON object)
    pub params: serde_json::Value,
}

/// MCP Response structure following JSON-RPC 2.0 specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    /// JSON-RPC version (should be "2.0")
    pub jsonrpc: String,
    /// Request identifier (echoes the request id)
    pub id: String,
    /// Result of the method call (if successful)
    pub result: Option<serde_json::Value>,
    /// Error information (if the call failed)
    pub error: Option<MCPError>,
}

/// MCP Error structure for error responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error data (optional)
    pub data: Option<serde_json::Value>,
}
