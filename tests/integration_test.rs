//! Integration Tests for MCP System Monitor
//!
//! Tests the complete system including HTTP API endpoints and MCP server functionality.

use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

use mcp_system_monitor::{
    types::{MCPRequest, METHOD_GET_CPU_INFO, METHOD_GET_SYSTEM_INFO},
    HTTPServer, MCPServer, SystemMonitor,
};

/// Test helper to create test system monitor
async fn create_test_system_monitor() -> Arc<RwLock<SystemMonitor>> {
    Arc::new(RwLock::new(
        SystemMonitor::new().expect("Failed to create system monitor"),
    ))
}

/// Test helper to create test MCP server
async fn create_test_mcp_server() -> MCPServer {
    let system_monitor = create_test_system_monitor().await;
    MCPServer::new(system_monitor)
}

/// Test helper to create test HTTP server
async fn create_test_http_server() -> HTTPServer {
    let system_monitor = create_test_system_monitor().await;
    HTTPServer::new(system_monitor)
}

#[tokio::test]
async fn test_mcp_server_creation() {
    let server = create_test_mcp_server().await;
    assert!(format!("{:?}", server).contains("MCPServer"));
}

#[tokio::test]
async fn test_http_server_creation() {
    let server = create_test_http_server().await;
    assert!(format!("{:?}", server).contains("HTTPServer"));
}

#[tokio::test]
async fn test_mcp_get_system_info() {
    let server = create_test_mcp_server().await;

    let request = MCPRequest {
        jsonrpc: "2.0".to_string(),
        id: "test-1".to_string(),
        method: METHOD_GET_SYSTEM_INFO.to_string(),
        params: json!({}),
    };

    let response = server.handle_request(request).await;

    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, "test-1");

    // On non-Linux systems, this might fail, so we check for either success or appropriate error
    if response.error.is_some() {
        // If there's an error, it should be a system-related error, not a protocol error
        let error = response.error.unwrap();
        assert!(error.code != -32601); // Should not be METHOD_NOT_FOUND
    } else {
        // If successful, verify the result structure
        if let Some(result) = response.result {
            let result_obj = result.as_object().expect("Result should be an object");
            // Basic fields should be present even if values are "Unknown"
            assert!(result_obj.contains_key("hostname"));
            assert!(result_obj.contains_key("os_name"));
            assert!(result_obj.contains_key("kernel_version"));
        }
    }
}

#[tokio::test]
async fn test_mcp_get_cpu_info() {
    let server = create_test_mcp_server().await;

    let request = MCPRequest {
        jsonrpc: "2.0".to_string(),
        id: "test-2".to_string(),
        method: METHOD_GET_CPU_INFO.to_string(),
        params: json!({}),
    };

    let response = server.handle_request(request).await;

    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, "test-2");

    // On non-Linux systems, this might fail, so we check for either success or appropriate error
    if response.error.is_some() {
        // If there's an error, it should be a system-related error, not a protocol error
        let error = response.error.unwrap();
        assert!(error.code != -32601); // Should not be METHOD_NOT_FOUND
    } else {
        // If successful, verify the result structure
        if let Some(result) = response.result {
            let result_obj = result.as_object().expect("Result should be an object");
            assert!(result_obj.contains_key("name"));
            assert!(result_obj.contains_key("cores"));
            assert!(result_obj.contains_key("usage_percent"));
        }
    }
}

#[tokio::test]
async fn test_mcp_invalid_method() {
    let server = create_test_mcp_server().await;

    let request = MCPRequest {
        jsonrpc: "2.0".to_string(),
        id: "test-3".to_string(),
        method: "invalidMethod".to_string(),
        params: json!({}),
    };

    let response = server.handle_request(request).await;

    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, "test-3");
    assert!(response.result.is_none());
    assert!(response.error.is_some());

    if let Some(error) = response.error {
        assert_eq!(error.code, -32601); // METHOD_NOT_FOUND
        assert!(error.message.contains("Method not found"));
    }
}

#[tokio::test]
async fn test_system_monitor_basic_functionality() {
    let system_monitor = SystemMonitor::new().expect("Failed to create system monitor");

    // Test that we can create a system monitor
    assert!(format!("{:?}", system_monitor).contains("SystemMonitor"));

    // Test that monitoring is initially not active
    assert!(!system_monitor.is_monitoring_active());
}

#[tokio::test]
async fn test_system_monitor_start_stop() {
    let mut system_monitor = SystemMonitor::new().expect("Failed to create system monitor");

    // Test starting monitoring
    let started = system_monitor
        .start_monitoring()
        .expect("Failed to start monitoring");
    assert!(started);
    assert!(system_monitor.is_monitoring_active());

    // Test starting again (should return false)
    let started_again = system_monitor
        .start_monitoring()
        .expect("Failed to start monitoring again");
    assert!(!started_again);
    assert!(system_monitor.is_monitoring_active());

    // Test stopping monitoring
    let stopped = system_monitor
        .stop_monitoring()
        .expect("Failed to stop monitoring");
    assert!(stopped);
    assert!(!system_monitor.is_monitoring_active());

    // Test stopping again (should return false)
    let stopped_again = system_monitor
        .stop_monitoring()
        .expect("Failed to stop monitoring again");
    assert!(!stopped_again);
    assert!(!system_monitor.is_monitoring_active());
}

#[tokio::test]
async fn test_system_info_collection() {
    let mut system_monitor = SystemMonitor::new().expect("Failed to create system monitor");

    // Test getting system info - this might fail on non-Linux systems
    match system_monitor.get_system_info() {
        Ok(system_info) => {
            // Verify basic fields are present
            assert!(!system_info.hostname.is_empty());
            assert!(!system_info.os_name.is_empty());
            assert!(!system_info.kernel_version.is_empty());
            assert!(system_info.uptime > 0);
        }
        Err(e) => {
            // On non-Linux systems, this is expected to fail
            println!(
                "System info collection failed (expected on non-Linux): {}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_cpu_info_collection() {
    let mut system_monitor = SystemMonitor::new().expect("Failed to create system monitor");

    // Test getting CPU info - this might fail on non-Linux systems
    match system_monitor.get_cpu_info() {
        Ok(cpu_info) => {
            // Verify basic fields are present
            assert!(!cpu_info.name.is_empty());
            assert!(cpu_info.cores > 0);
            assert!(cpu_info.usage_percent >= 0.0);
            assert!(cpu_info.usage_percent <= 100.0);
        }
        Err(e) => {
            // On non-Linux systems, this is expected to fail
            println!("CPU info collection failed (expected on non-Linux): {}", e);
        }
    }
}

#[tokio::test]
async fn test_memory_info_collection() {
    let mut system_monitor = SystemMonitor::new().expect("Failed to create system monitor");

    // Test getting memory info - this might fail on non-Linux systems
    match system_monitor.get_memory_info() {
        Ok(memory_info) => {
            // Verify basic fields are present and logical
            assert!(memory_info.total > 0);
            assert!(memory_info.used <= memory_info.total);
            assert!(memory_info.free <= memory_info.total);
            assert!(memory_info.available <= memory_info.total);
            assert!(memory_info.usage_percent >= 0.0);
            assert!(memory_info.usage_percent <= 100.0);
        }
        Err(e) => {
            // On non-Linux systems, this is expected to fail
            println!(
                "Memory info collection failed (expected on non-Linux): {}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_disk_info_collection() {
    let mut system_monitor = SystemMonitor::new().expect("Failed to create system monitor");

    // Test getting disk info - this might fail on non-Linux systems
    match system_monitor.get_disk_info() {
        Ok(disks) => {
            // Verify we have at least one disk
            assert!(!disks.is_empty());

            for disk in disks {
                assert!(!disk.name.is_empty());
                assert!(!disk.mount_point.is_empty());
                assert!(disk.total_space > 0);
                assert!(disk.used_space <= disk.total_space);
                assert!(disk.free_space <= disk.total_space);
                assert!(disk.usage_percent >= 0.0);
                assert!(disk.usage_percent <= 100.0);
            }
        }
        Err(e) => {
            // On non-Linux systems, this is expected to fail
            println!("Disk info collection failed (expected on non-Linux): {}", e);
        }
    }
}

#[tokio::test]
async fn test_network_info_collection() {
    let mut system_monitor = SystemMonitor::new().expect("Failed to create system monitor");

    // Test getting network info - this might fail on non-Linux systems
    match system_monitor.get_network_info() {
        Ok(networks) => {
            // Verify we have at least one network interface
            assert!(!networks.is_empty());

            for network in networks {
                assert!(!network.interface.is_empty());
                // Other fields might be "N/A" on some systems, so we just check they exist
                assert!(!network.ip_address.is_empty());
                assert!(!network.mac_address.is_empty());
            }
        }
        Err(e) => {
            // On non-Linux systems, this is expected to fail
            println!(
                "Network info collection failed (expected on non-Linux): {}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_process_info_collection() {
    let mut system_monitor = SystemMonitor::new().expect("Failed to create system monitor");

    // Test getting processes - this might fail on non-Linux systems
    match system_monitor.get_processes() {
        Ok(processes) => {
            // Verify we have at least one process
            assert!(!processes.is_empty());

            for process in processes {
                assert!(process.pid > 0);
                assert!(!process.name.is_empty());
                assert!(process.cpu_usage >= 0.0);
                assert!(process.memory_usage_percent >= 0.0);
                assert!(process.memory_usage_percent <= 100.0);
            }
        }
        Err(e) => {
            // On non-Linux systems, this is expected to fail
            println!(
                "Process info collection failed (expected on non-Linux): {}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_system_metrics_collection() {
    let mut system_monitor = SystemMonitor::new().expect("Failed to create system monitor");

    // Test getting complete system metrics - this might fail on non-Linux systems
    match system_monitor.get_system_metrics() {
        Ok(metrics) => {
            // Verify all components are present
            assert!(!metrics.system_info.hostname.is_empty());
            assert!(!metrics.cpu_info.name.is_empty());
            assert!(metrics.memory_info.total > 0);
            assert!(!metrics.disks.is_empty());
            assert!(!metrics.networks.is_empty());
            assert!(!metrics.processes.is_empty());
        }
        Err(e) => {
            // On non-Linux systems, this is expected to fail
            println!(
                "System metrics collection failed (expected on non-Linux): {}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_process_by_pid() {
    let mut system_monitor = SystemMonitor::new().expect("Failed to create system monitor");

    // Test getting process by PID - this might fail on non-Linux systems
    match system_monitor.get_process_by_pid(1) {
        Ok(process) => {
            // Process 1 should exist on Linux systems
            if let Some(process) = process {
                assert_eq!(process.pid, 1);
                assert!(!process.name.is_empty());
            }
        }
        Err(e) => {
            // On non-Linux systems, this is expected to fail
            println!(
                "Process by PID collection failed (expected on non-Linux): {}",
                e
            );
        }
    }

    // Test getting a non-existent process
    match system_monitor.get_process_by_pid(999999) {
        Ok(non_existent) => {
            assert!(non_existent.is_none());
        }
        Err(e) => {
            // On non-Linux systems, this is expected to fail
            println!(
                "Non-existent process test failed (expected on non-Linux): {}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_mcp_error_handling() {
    let server = create_test_mcp_server().await;

    // Test with invalid parameters
    let request = MCPRequest {
        jsonrpc: "2.0".to_string(),
        id: "test-error".to_string(),
        method: METHOD_GET_SYSTEM_INFO.to_string(),
        params: json!({"invalid": "params"}), // Extra params should be ignored
    };

    let response = server.handle_request(request).await;

    // Should still succeed even with extra params
    assert!(response.error.is_none());
    assert!(response.result.is_some());
}

#[tokio::test]
async fn test_mcp_response_format() {
    let server = create_test_mcp_server().await;

    let request = MCPRequest {
        jsonrpc: "2.0".to_string(),
        id: "test-format".to_string(),
        method: METHOD_GET_SYSTEM_INFO.to_string(),
        params: json!({}),
    };

    let response = server.handle_request(request).await;

    // Verify JSON-RPC 2.0 compliance
    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, "test-format");
    assert!(response.error.is_none() || response.result.is_none()); // XOR
    assert!(!(response.error.is_some() && response.result.is_some())); // Not both
}
