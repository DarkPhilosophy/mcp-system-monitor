use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

mod server;
mod system_monitor;
mod types;
mod http_server;
mod stdio_server;

use server::MCPServer;
use system_monitor::SystemMonitor;
use http_server::HTTPServer;
use stdio_server::StdioServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Check if running in stdio mode first
    let args: Vec<String> = std::env::args().collect();
    let use_stdio = args.contains(&"--stdio".to_string());
    
    // Initialize logging - redirect to stderr for both modes
    let subscriber = tracing_subscriber::fmt().with_writer(std::io::stderr);
    let subscriber = if use_stdio {
        subscriber.with_max_level(tracing::Level::ERROR)
    } else {
        subscriber.with_max_level(tracing::Level::INFO)
    };
    let _ = subscriber.try_init();
    
    if !use_stdio {
        info!("Starting MCP System Monitor Server...");
    }
    
    // Initialize system monitor
    let system_monitor = Arc::new(RwLock::new(SystemMonitor::new()?));
    
    if use_stdio {
        // Don't initialize logging for stdio mode - interferes with MCP protocol
        let stdio_server = StdioServer::new(system_monitor);
        if let Err(e) = stdio_server.run().await {
            return Err(anyhow::anyhow!("STDIO server failed: {}", e));
        }
    } else {
        // Create and start MCP server
        let _mcp_server = MCPServer::new(system_monitor.clone());
        
        // Create HTTP server
        let http_server = HTTPServer::new(system_monitor);
        
        info!("MCP Server initialized successfully");
        info!("Starting HTTP server on port 57996...");

        // Run HTTP server
        if let Err(e) = http_server.run(57996).await {
            error!("HTTP server error: {}", e);
            return Err(anyhow::anyhow!("HTTP server failed: {}", e));
        }
    }
    
    Ok(())
} 
