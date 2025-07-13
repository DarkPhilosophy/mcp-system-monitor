//! Core System Monitor Implementation
//!
//! Main SystemMonitor struct and public API methods.

use anyhow::Result;
use chrono::{DateTime, Utc};
use tracing::info;

use super::linux::LinuxSystemInfo;
use crate::types::*;

/// System Monitor for collecting real-time system information
///
/// Provides methods to collect system metrics including CPU, memory, disk,
/// network, and process information from Linux systems.
#[derive(Debug)]
pub struct SystemMonitor {
    /// Whether continuous monitoring is currently active
    monitoring_active: bool,
    /// Timestamp of the last data refresh
    last_update: DateTime<Utc>,
    /// Linux-specific system information collector
    linux_info: LinuxSystemInfo,
}

impl SystemMonitor {
    /// Creates new SystemMonitor instance
    pub fn new() -> Result<Self> {
        let linux_info = LinuxSystemInfo::new()?;

        Ok(Self {
            monitoring_active: false,
            last_update: Utc::now(),
            linux_info,
        })
    }

    /// Refreshes the last update timestamp
    pub fn refresh(&mut self) {
        self.last_update = Utc::now();
    }

    /// Gets comprehensive system information
    pub fn get_system_info(&mut self) -> Result<SystemInfo> {
        self.refresh();
        self.linux_info.get_system_info()
    }

    /// Gets CPU information and usage statistics
    pub fn get_cpu_info(&mut self) -> Result<CPUInfo> {
        self.refresh();
        self.linux_info.get_cpu_info()
    }

    /// Gets memory information including RAM and swap usage
    pub fn get_memory_info(&mut self) -> Result<MemoryInfo> {
        self.refresh();
        self.linux_info.get_memory_info()
    }

    /// Gets disk information for all mounted filesystems
    pub fn get_disk_info(&mut self) -> Result<Vec<DiskInfo>> {
        self.refresh();
        self.linux_info.get_disk_info()
    }

    /// Gets network interface information and statistics
    pub fn get_network_info(&mut self) -> Result<Vec<NetworkInfo>> {
        self.refresh();
        self.linux_info.get_network_info()
    }

    /// Gets information about all running processes
    pub fn get_processes(&mut self) -> Result<Vec<ProcessInfo>> {
        self.refresh();
        self.linux_info.get_processes()
    }

    /// Gets information about a specific process by PID
    pub fn get_process_by_pid(&mut self, pid: u32) -> Result<Option<ProcessInfo>> {
        self.refresh();
        self.linux_info.get_process_by_pid(pid)
    }

    /// Gets a complete snapshot of all system metrics
    pub fn get_system_metrics(&mut self) -> Result<SystemMetrics> {
        self.refresh();

        let system_info = self.get_system_info()?;
        let cpu_info = self.get_cpu_info()?;
        let memory_info = self.get_memory_info()?;
        let disks = self.get_disk_info()?;
        let networks = self.get_network_info()?;
        let processes = self.get_processes()?;

        Ok(SystemMetrics {
            timestamp: Utc::now(),
            system_info,
            cpu_info,
            memory_info,
            disks,
            networks,
            processes,
        })
    }

    /// Starts continuous monitoring
    pub fn start_monitoring(&mut self) -> Result<bool> {
        if self.monitoring_active {
            info!("Monitoring already active");
            Ok(false)
        } else {
            self.monitoring_active = true;
            info!("Monitoring started");
            Ok(true)
        }
    }

    /// Stops continuous monitoring
    pub fn stop_monitoring(&mut self) -> Result<bool> {
        if self.monitoring_active {
            self.monitoring_active = false;
            info!("Monitoring stopped");
            Ok(true)
        } else {
            info!("Monitoring not active");
            Ok(false)
        }
    }

    /// Checks if monitoring is currently active
    pub fn is_monitoring_active(&self) -> bool {
        self.monitoring_active
    }

    /// Gets the timestamp of the last data refresh
    pub fn last_update(&self) -> DateTime<Utc> {
        self.last_update
    }
}
