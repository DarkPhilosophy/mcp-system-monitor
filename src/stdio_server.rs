//! STDIO Server Implementation
//!
//! Handles JSON-RPC requests over stdin/stdout for local MCP integration.

use serde_json::json;
use std::io::{BufRead, BufReader, Read, Write};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::error;

use crate::server::MCPServer;
use crate::system_monitor::SystemMonitor;
use crate::types::*;

/// STDIO Server for stdin/stdout communication
pub struct StdioServer {
    mcp_server: MCPServer,
}

impl StdioServer {
    /// Creates new STDIO server instance
    pub fn new(system_monitor: Arc<RwLock<SystemMonitor>>) -> Self {
        Self {
            mcp_server: MCPServer::new(system_monitor),
        }
    }

    /// Runs STDIO server, reading from stdin and writing to stdout
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        let mut reader = BufReader::new(stdin);

        loop {
            let mut content_length = None;

            loop {
                let mut header = String::new();
                let bytes = reader.read_line(&mut header)?;
                if bytes == 0 {
                    return Ok(());
                }

                if header == "\r\n" || header == "\n" {
                    break;
                }

                if let Some(value) = header.strip_prefix("Content-Length:") {
                    if let Ok(length) = value.trim().parse::<usize>() {
                        content_length = Some(length);
                    }
                }
            }

            let Some(length) = content_length else {
                continue;
            };

            let mut payload = vec![0u8; length];
            reader.read_exact(&mut payload)?;
            let json_str = String::from_utf8(payload)?;

            match serde_json::from_str::<MCPRequest>(&json_str) {
                Ok(request) => {
                    let response = self.mcp_server.handle_request(request).await;
                    let response_json = serde_json::to_string(&response)?;
                    let mut out = stdout.lock();
                    write!(out, "Content-Length: {}\r\n\r\n{}", response_json.len(), response_json)?;
                    out.flush()?;
                }
                Err(e) => {
                    error!("Failed to parse JSON-RPC request: {}", e);
                    let error_response = json!({
                        "jsonrpc": "2.0",
                        "error": {
                            "code": -32700,
                            "message": "Parse error"
                        }
                    });
                    let error_json = serde_json::to_string(&error_response)?;
                    let mut out = stdout.lock();
                    write!(out, "Content-Length: {}\r\n\r\n{}", error_json.len(), error_json)?;
                    out.flush()?;
                }
            }
        }
    }
}
