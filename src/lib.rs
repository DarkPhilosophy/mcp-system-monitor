//! MCP System Monitor Library
//!
//! Comprehensive system monitoring library for Linux servers with MCP protocol
//! and HTTP REST API support.
//!
//! ## Features
//!
//! - Linux system monitoring with real-time data collection
//! - MCP protocol support for AI agent integration
//! - HTTP REST API for system information access
//! - CPU, memory, disk, network, and process monitoring
//! - Async support with Tokio
//!
//! ## Example
//!
//! ```rust
//! use mcp_system_monitor::{SystemMonitor, MCPServer, HTTPServer};
//! use std::sync::Arc;
//! use tokio::sync::RwLock;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let system_monitor = Arc::new(RwLock::new(SystemMonitor::new()?));
//!     let mcp_server = MCPServer::new(system_monitor.clone());
//!     let http_server = HTTPServer::new(system_monitor);
//!     http_server.run(8080).await?;
//!     Ok(())
//! }
//! ```

pub mod http_server;
pub mod server;
pub mod system_monitor;
pub mod types;

// Re-export main types
pub use http_server::HTTPServer;
pub use server::MCPServer;
pub use system_monitor::SystemMonitor;
pub use types::*;
