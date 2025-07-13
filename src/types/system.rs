//! System Monitoring Types
//!
//! Data structures for system information, metrics, and monitoring data.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// System information including hostname, OS details, and uptime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// System hostname
    pub hostname: String,
    /// Operating system name (e.g., "Ubuntu", "CentOS")
    pub os_name: String,
    /// Operating system version
    pub os_version: String,
    /// Kernel version
    pub kernel_version: String,
    /// System uptime in seconds
    pub uptime: u64,
    /// System boot time
    pub boot_time: DateTime<Utc>,
}

/// CPU information including usage, frequency, and specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUInfo {
    /// CPU model name
    pub name: String,
    /// CPU brand information
    pub brand: String,
    /// CPU frequency in MHz
    pub frequency: u64,
    /// Number of CPU cores
    pub cores: u32,
    /// CPU usage percentage (0.0 - 100.0)
    pub usage_percent: f32,
    /// CPU temperature in Celsius (if available)
    pub temperature: Option<f32>,
}

/// Memory information including RAM and swap usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// Total physical memory in bytes
    pub total: u64,
    /// Used physical memory in bytes
    pub used: u64,
    /// Free physical memory in bytes
    pub free: u64,
    /// Available physical memory in bytes
    pub available: u64,
    /// Total swap space in bytes
    pub swap_total: u64,
    /// Used swap space in bytes
    pub swap_used: u64,
    /// Free swap space in bytes
    pub swap_free: u64,
    /// Memory usage percentage (0.0 - 100.0)
    pub usage_percent: f32,
    /// Swap usage percentage (0.0 - 100.0)
    pub swap_usage_percent: f32,
}

/// Disk information including storage usage and file system details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    /// Device name
    pub name: String,
    /// Mount point
    pub mount_point: String,
    /// File system type
    pub file_system: String,
    /// Total disk space in bytes
    pub total_space: u64,
    /// Used disk space in bytes
    pub used_space: u64,
    /// Free disk space in bytes
    pub free_space: u64,
    /// Disk usage percentage (0.0 - 100.0)
    pub usage_percent: f32,
}

/// Network interface information and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// Network interface name
    pub interface: String,
    /// IP address
    pub ip_address: String,
    /// MAC address
    pub mac_address: String,
    /// Total bytes received
    pub bytes_received: u64,
    /// Total bytes transmitted
    pub bytes_transmitted: u64,
    /// Total packets received
    pub packets_received: u64,
    /// Total packets transmitted
    pub packets_transmitted: u64,
    /// Total receive errors
    pub errors_received: u64,
    /// Total transmit errors
    pub errors_transmitted: u64,
}

/// Process information including resource usage and details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub name: String,
    /// Full command line
    pub command: String,
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Memory usage percentage
    pub memory_usage_percent: f32,
    /// Process status
    pub status: String,
    /// Process start time
    pub start_time: DateTime<Utc>,
    /// Process owner
    pub user: String,
    /// Process priority
    pub priority: i32,
}

/// Complete system metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Timestamp of the metrics collection
    pub timestamp: DateTime<Utc>,
    /// System information
    pub system_info: SystemInfo,
    /// CPU information
    pub cpu_info: CPUInfo,
    /// Memory information
    pub memory_info: MemoryInfo,
    /// Disk information for all mounted filesystems
    pub disks: Vec<DiskInfo>,
    /// Network information for all interfaces
    pub networks: Vec<NetworkInfo>,
    /// Process information for all running processes
    pub processes: Vec<ProcessInfo>,
}
