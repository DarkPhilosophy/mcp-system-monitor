//! MCP Protocol Types
//!
//! Core JSON-RPC protocol structures for MCP communication.

use serde::{Deserialize, Serialize};
use serde_json::json;

/// MCP Request structure following JSON-RPC 2.0 specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRequest {
    /// JSON-RPC version (should be "2.0")
    pub jsonrpc: String,
    /// Unique request identifier (can be string, number, or null per JSON-RPC 2.0; absent for notifications)
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[serde(serialize_with = "serialize_id_opt")]
    #[serde(deserialize_with = "deserialize_id_opt")]
    pub id: Option<String>,
    /// Method name to be invoked
    pub method: String,
    /// Method parameters (JSON object)
    #[serde(default)]
    pub params: serde_json::Value,
}

/// Custom deserializer for optional id field to handle string, number, or null
fn deserialize_id_opt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserializer as _;
    
    // Use visit_some to get the value or default to None
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum IdValue {
        String(String),
        Number(i64),
        Null,
    }
    
    let opt: Option<IdValue> = Option::deserialize(deserializer)?;
    Ok(opt.map(|v| match v {
        IdValue::String(s) => s,
        IdValue::Number(n) => n.to_string(),
        IdValue::Null => "null".to_string(),
    }))
}

/// Custom serializer for optional id field
fn serialize_id_opt<S>(id: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match id {
        Some(id_str) => {
            // Try to parse as number first  
            if let Ok(n) = id_str.parse::<i64>() {
                n.serialize(serializer)
            } else if id_str == "null" {
                serializer.serialize_none()
            } else {
                id_str.serialize(serializer)
            }
        },
        None => serializer.serialize_none()
    }
}

/// MCP Response structure following JSON-RPC 2.0 specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    /// JSON-RPC version (should be "2.0")
    pub jsonrpc: String,
    /// Request identifier (echoes the request id; absent for notifications)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Result of the method call (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error information (if the call failed)
    #[serde(skip_serializing_if = "Option::is_none")]
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
