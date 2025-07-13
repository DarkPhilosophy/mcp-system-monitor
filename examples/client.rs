//! MCP System Monitor Client Example
//!
//! Demonstrates how to use the MCP System Monitor HTTP API
//! to retrieve system information from a running server.

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

/// Configuration for the client
#[derive(Debug)]
struct ClientConfig {
    /// Base URL of the MCP server
    base_url: String,
    /// HTTP client timeout
    timeout: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8080".to_string(),
            timeout: Duration::from_secs(10),
        }
    }
}

/// MCP System Monitor Client
///
/// Provides methods to interact with the MCP System Monitor HTTP API endpoints.
#[derive(Debug)]
struct MCPClient {
    /// HTTP client for making requests
    http_client: Client,
    /// Client configuration
    config: ClientConfig,
}

impl MCPClient {
    /// Creates new MCP client instance
    pub fn new(config: ClientConfig) -> Result<Self> {
        let http_client = Client::builder().timeout(config.timeout).build()?;

        Ok(Self {
            http_client,
            config,
        })
    }

    /// Creates new MCP client with default configuration
    pub fn new_default() -> Result<Self> {
        Self::new(ClientConfig::default())
    }

    /// Performs health check on the server
    pub async fn health_check(&self) -> Result<Value> {
        let url = format!("{}/health", self.config.base_url);
        let response = self.http_client.get(&url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            Ok(data)
        } else {
            Err(anyhow!(
                "Health check failed with status: {}",
                response.status()
            ))
        }
    }

    /// Gets system information
    pub async fn get_system_info(&self) -> Result<Value> {
        let url = format!("{}/api/system/info", self.config.base_url);
        let response = self.http_client.get(&url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            Ok(data)
        } else {
            Err(anyhow!("Failed to get system info: {}", response.status()))
        }
    }

    /// Gets CPU information
    pub async fn get_cpu_info(&self) -> Result<Value> {
        let url = format!("{}/api/system/cpu", self.config.base_url);
        let response = self.http_client.get(&url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            Ok(data)
        } else {
            Err(anyhow!("Failed to get CPU info: {}", response.status()))
        }
    }

    /// Gets memory information
    pub async fn get_memory_info(&self) -> Result<Value> {
        let url = format!("{}/api/system/memory", self.config.base_url);
        let response = self.http_client.get(&url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            Ok(data)
        } else {
            Err(anyhow!("Failed to get memory info: {}", response.status()))
        }
    }

    /// Gets disk information
    pub async fn get_disk_info(&self) -> Result<Value> {
        let url = format!("{}/api/system/disks", self.config.base_url);
        let response = self.http_client.get(&url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            Ok(data)
        } else {
            Err(anyhow!("Failed to get disk info: {}", response.status()))
        }
    }

    /// Gets network information
    pub async fn get_network_info(&self) -> Result<Value> {
        let url = format!("{}/api/system/networks", self.config.base_url);
        let response = self.http_client.get(&url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            Ok(data)
        } else {
            Err(anyhow!("Failed to get network info: {}", response.status()))
        }
    }

    /// Gets all processes
    pub async fn get_processes(&self) -> Result<Value> {
        let url = format!("{}/api/system/processes", self.config.base_url);
        let response = self.http_client.get(&url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            Ok(data)
        } else {
            Err(anyhow!("Failed to get processes: {}", response.status()))
        }
    }

    /// Gets specific process by PID
    pub async fn get_process_by_pid(&self, pid: u32) -> Result<Value> {
        let url = format!("{}/api/system/processes/{}", self.config.base_url, pid);
        let response = self.http_client.get(&url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            Ok(data)
        } else {
            Err(anyhow!(
                "Failed to get process by PID {}: {}",
                pid,
                response.status()
            ))
        }
    }

    /// Gets system metrics
    pub async fn get_system_metrics(&self) -> Result<Value> {
        let url = format!("{}/api/system/metrics", self.config.base_url);
        let response = self.http_client.get(&url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            Ok(data)
        } else {
            Err(anyhow!(
                "Failed to get system metrics: {}",
                response.status()
            ))
        }
    }

    /// Starts monitoring
    pub async fn start_monitoring(&self) -> Result<Value> {
        let url = format!("{}/api/monitoring/start", self.config.base_url);
        let response = self.http_client.post(&url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            Ok(data)
        } else {
            Err(anyhow!("Failed to start monitoring: {}", response.status()))
        }
    }

    /// Stops monitoring
    pub async fn stop_monitoring(&self) -> Result<Value> {
        let url = format!("{}/api/monitoring/stop", self.config.base_url);
        let response = self.http_client.post(&url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            Ok(data)
        } else {
            Err(anyhow!("Failed to stop monitoring: {}", response.status()))
        }
    }

    /// Gets monitoring status
    pub async fn get_monitoring_status(&self) -> Result<Value> {
        let url = format!("{}/api/monitoring/status", self.config.base_url);
        let response = self.http_client.get(&url).send().await?;

        if response.status().is_success() {
            let data: Value = response.json().await?;
            Ok(data)
        } else {
            Err(anyhow!(
                "Failed to get monitoring status: {}",
                response.status()
            ))
        }
    }

    /// Prints JSON data with title
    fn print_json(&self, title: &str, data: &Value) {
        println!("\n=== {} ===", title);
        println!("{}", serde_json::to_string_pretty(data).unwrap());
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("MCP System Monitor Client Example");
    println!("=================================");

    // Create client with custom configuration
    let config = ClientConfig {
        base_url: "http://localhost:8080".to_string(),
        timeout: Duration::from_secs(10),
    };

    let client = MCPClient::new(config)?;

    // Test health check first
    println!("\n1. Testing health check...");
    match client.health_check().await {
        Ok(health) => {
            client.print_json("Health Check", &health);
            println!("✓ Server is healthy!");
        }
        Err(e) => {
            println!("✗ Health check failed: {}", e);
            println!("Make sure the MCP server is running on http://localhost:8080");
            return Err(e);
        }
    }

    // Example 1: Get system information
    println!("\n2. Getting system information...");
    match client.get_system_info().await {
        Ok(info) => client.print_json("System Information", &info),
        Err(e) => println!("✗ Failed to get system info: {}", e),
    }

    // Example 2: Get CPU information
    println!("\n3. Getting CPU information...");
    match client.get_cpu_info().await {
        Ok(info) => client.print_json("CPU Information", &info),
        Err(e) => println!("✗ Failed to get CPU info: {}", e),
    }

    // Example 3: Get memory information
    println!("\n4. Getting memory information...");
    match client.get_memory_info().await {
        Ok(info) => client.print_json("Memory Information", &info),
        Err(e) => println!("✗ Failed to get memory info: {}", e),
    }

    // Example 4: Get disk information
    println!("\n5. Getting disk information...");
    match client.get_disk_info().await {
        Ok(info) => client.print_json("Disk Information", &info),
        Err(e) => println!("✗ Failed to get disk info: {}", e),
    }

    // Example 5: Get network information
    println!("\n6. Getting network information...");
    match client.get_network_info().await {
        Ok(info) => client.print_json("Network Information", &info),
        Err(e) => println!("✗ Failed to get network info: {}", e),
    }

    // Example 6: Get processes (limit to first 10 for readability)
    println!("\n7. Getting processes (showing first 10)...");
    match client.get_processes().await {
        Ok(processes) => {
            if let Some(processes_array) = processes.as_array() {
                let limited_processes = json!(processes_array.iter().take(10).collect::<Vec<_>>());
                client.print_json("Processes (First 10)", &limited_processes);
                println!("Total processes: {}", processes_array.len());
            } else {
                client.print_json("Processes", &processes);
            }
        }
        Err(e) => println!("✗ Failed to get processes: {}", e),
    }

    // Example 7: Get specific process by PID
    println!("\n8. Getting process by PID (PID 1)...");
    match client.get_process_by_pid(1).await {
        Ok(process) => client.print_json("Process PID 1", &process),
        Err(e) => println!("✗ Failed to get process by PID: {}", e),
    }

    // Example 8: Get complete system metrics
    println!("\n9. Getting complete system metrics...");
    match client.get_system_metrics().await {
        Ok(metrics) => {
            println!("=== Complete System Metrics ===");
            println!(
                "System Info: {}",
                serde_json::to_string_pretty(&metrics["system_info"]).unwrap()
            );
            println!(
                "CPU Info: {}",
                serde_json::to_string_pretty(&metrics["cpu_info"]).unwrap()
            );
            println!(
                "Memory Info: {}",
                serde_json::to_string_pretty(&metrics["memory_info"]).unwrap()
            );
            println!(
                "Disks: {} items",
                metrics["disks"].as_array().unwrap().len()
            );
            println!(
                "Networks: {} items",
                metrics["networks"].as_array().unwrap().len()
            );
            println!(
                "Processes: {} items",
                metrics["processes"].as_array().unwrap().len()
            );
        }
        Err(e) => println!("✗ Failed to get system metrics: {}", e),
    }

    // Example 9: Start monitoring
    println!("\n10. Starting monitoring...");
    match client.start_monitoring().await {
        Ok(result) => client.print_json("Start Monitoring", &result),
        Err(e) => println!("✗ Failed to start monitoring: {}", e),
    }

    // Example 10: Get monitoring status
    println!("\n11. Getting monitoring status...");
    match client.get_monitoring_status().await {
        Ok(status) => client.print_json("Monitoring Status", &status),
        Err(e) => println!("✗ Failed to get monitoring status: {}", e),
    }

    // Example 11: Stop monitoring
    println!("\n12. Stopping monitoring...");
    match client.stop_monitoring().await {
        Ok(result) => client.print_json("Stop Monitoring", &result),
        Err(e) => println!("✗ Failed to stop monitoring: {}", e),
    }

    println!("\n✓ Client example completed successfully!");
    println!("This demonstrates how to use the MCP System Monitor HTTP API.");
    println!("You can use this client as a reference for integrating with your own applications.");

    Ok(())
}
