use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

mod server;
mod system_monitor;
mod types;
mod http_server;

use server::MCPServer;
use system_monitor::SystemMonitor;
use http_server::HTTPServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting MCP System Monitor Server...");
    
    // Initialize system monitor
    let system_monitor = Arc::new(RwLock::new(SystemMonitor::new()?));
    
    // Create and start MCP server
    let mcp_server = MCPServer::new(system_monitor.clone());
    
    // Create HTTP server
    let http_server = HTTPServer::new(system_monitor);
    
    info!("MCP Server initialized successfully");
    info!("Starting HTTP server on port 8080...");
    
    // Run HTTP server
    if let Err(e) = http_server.run(8080).await {
        error!("HTTP server error: {}", e);
        return Err(anyhow::anyhow!("HTTP server failed: {}", e));
    }
    
    Ok(())
} 