//! Linux System Information Collector
//!
//! This module contains Linux-specific implementations for collecting
//! system information using Linux commands and procfs.

use anyhow::{anyhow, Result};
use chrono::Utc;
use num_cpus;
use std::env;
use std::process::Command;

use super::helpers::*;
use crate::types::*;

/// Linux-specific system information collector
///
/// This struct provides methods to collect system information from Linux systems
/// using standard Linux commands and procfs files.
#[derive(Debug)]
pub struct LinuxSystemInfo;

impl LinuxSystemInfo {
    /// Creates a new LinuxSystemInfo instance
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Gets system information including hostname, OS details, and uptime
    pub fn get_system_info(&self) -> Result<SystemInfo> {
        // Get hostname
        let hostname = self.get_hostname()?;

        // Get OS information
        let (os_name, os_version) = self.get_os_info()?;

        // Get kernel version
        let kernel_version = self.get_kernel_version()?;

        // Get uptime
        let uptime = self.get_uptime()?;
        let boot_time = Utc::now() - chrono::Duration::seconds(uptime as i64);

        Ok(SystemInfo {
            hostname,
            os_name,
            os_version,
            kernel_version,
            uptime,
            boot_time,
        })
    }

    /// Gets CPU information and usage statistics
    pub fn get_cpu_info(&self) -> Result<CPUInfo> {
        // Get CPU cores
        let cores = num_cpus::get() as u32;

        // Get CPU model name
        let name = self.get_cpu_model()?;
        let brand = name.clone();

        // Get CPU frequency
        let frequency = self.get_cpu_frequency()?;

        // Get CPU usage from /proc/stat
        let usage_percent = self.get_cpu_usage()?;

        // Get CPU temperature (if available)
        let temperature = self.get_cpu_temperature();

        Ok(CPUInfo {
            name,
            brand,
            frequency,
            cores,
            usage_percent,
            temperature,
        })
    }

    /// Gets memory information including RAM and swap usage
    pub fn get_memory_info(&self) -> Result<MemoryInfo> {
        let meminfo = self.read_proc_meminfo()?;

        let mut total = 0u64;
        let mut free = 0u64;
        let mut available = 0u64;
        let mut swap_total = 0u64;
        let mut swap_free = 0u64;

        for line in meminfo.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let value: u64 = parts[1].parse().unwrap_or(0) * 1024; // Convert KB to bytes
                match parts[0] {
                    "MemTotal:" => total = value,
                    "MemFree:" => free = value,
                    "MemAvailable:" => available = value,
                    "SwapTotal:" => swap_total = value,
                    "SwapFree:" => swap_free = value,
                    _ => {}
                }
            }
        }

        let used = total - available;
        let swap_used = swap_total - swap_free;

        let usage_percent = calculate_percentage(used, total);
        let swap_usage_percent = calculate_percentage(swap_used, swap_total);

        Ok(MemoryInfo {
            total,
            used,
            free,
            available,
            swap_total,
            swap_used,
            swap_free,
            usage_percent,
            swap_usage_percent,
        })
    }

    /// Gets disk information for all mounted filesystems
    pub fn get_disk_info(&self) -> Result<Vec<DiskInfo>> {
        let output = Command::new("df")
            .arg("-h")
            .arg("--output=source,target,fstype,size,used,avail")
            .output()
            .map_err(|e| anyhow!("Failed to execute df command: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut disks = Vec::new();

        for line in output_str.lines().skip(1) {
            // Skip header
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                let name = parts[0].to_string();
                let mount_point = parts[1].to_string();
                let file_system = parts[2].to_string();

                // Parse sizes (remove 'G', 'M', etc. and convert to bytes)
                let total_space = parse_size(parts[3])?;
                let used_space = parse_size(parts[4])?;
                let free_space = parse_size(parts[5])?;

                let usage_percent = calculate_percentage(used_space, total_space);

                disks.push(DiskInfo {
                    name,
                    mount_point,
                    file_system,
                    total_space,
                    used_space,
                    free_space,
                    usage_percent,
                });
            }
        }

        Ok(disks)
    }

    /// Gets network interface information and statistics
    pub fn get_network_info(&self) -> Result<Vec<NetworkInfo>> {
        let output = Command::new("cat")
            .arg("/proc/net/dev")
            .output()
            .map_err(|e| anyhow!("Failed to read /proc/net/dev: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut networks = Vec::new();

        for line in output_str.lines().skip(2) {
            // Skip header lines
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 17 {
                let interface = parts[0].trim_end_matches(':').to_string();

                // Skip loopback interface
                if interface == "lo" {
                    continue;
                }

                let bytes_received = safe_parse_u64(parts[1]);
                let packets_received = safe_parse_u64(parts[2]);
                let errors_received = safe_parse_u64(parts[3]);
                let bytes_transmitted = safe_parse_u64(parts[9]);
                let packets_transmitted = safe_parse_u64(parts[10]);
                let errors_transmitted = safe_parse_u64(parts[11]);

                let ip_address = self
                    .get_interface_ip(&interface)
                    .unwrap_or_else(|_| "N/A".to_string());
                let mac_address = self
                    .get_interface_mac(&interface)
                    .unwrap_or_else(|_| "N/A".to_string());

                networks.push(NetworkInfo {
                    interface,
                    ip_address,
                    mac_address,
                    bytes_received,
                    bytes_transmitted,
                    packets_received,
                    packets_transmitted,
                    errors_received,
                    errors_transmitted,
                });
            }
        }

        Ok(networks)
    }

    /// Gets information about all running processes
    pub fn get_processes(&self) -> Result<Vec<ProcessInfo>> {
        let output = Command::new("ps")
            .args(&[
                "-eo",
                "pid,ppid,user,pcpu,pmem,vsz,rss,stat,etime,comm,args,pri",
            ])
            .output()
            .map_err(|e| anyhow!("Failed to execute ps command: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut processes = Vec::new();

        for line in output_str.lines().skip(1) {
            // Skip header
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 12 {
                let pid = safe_parse_u32(parts[0]);
                let user = parts[2].to_string();
                let cpu_usage = safe_parse_f32(parts[3]);
                let memory_usage_percent = safe_parse_f32(parts[4]);
                let _virtual_memory = safe_parse_u64(parts[5]);
                let physical_memory = safe_parse_u64(parts[6]) * 1024; // Convert KB to bytes
                let status = parts[7].to_string();
                let etime = parts[8].to_string();
                let name = parts[9].to_string();
                let command = parts[10..].join(" ");
                let priority = safe_parse_i32(parts[11]);

                let start_time = parse_etime(&etime)?;
                let memory_usage = physical_memory;

                processes.push(ProcessInfo {
                    pid,
                    name,
                    command,
                    cpu_usage,
                    memory_usage,
                    memory_usage_percent,
                    status,
                    start_time,
                    user,
                    priority,
                });
            }
        }

        Ok(processes)
    }

    /// Gets information about a specific process by PID
    pub fn get_process_by_pid(&self, pid: u32) -> Result<Option<ProcessInfo>> {
        let output = Command::new("ps")
            .args(&[
                "-p",
                &pid.to_string(),
                "-o",
                "pid,ppid,user,pcpu,pmem,vsz,rss,stat,etime,comm,args,pri",
            ])
            .output()
            .map_err(|e| anyhow!("Failed to execute ps command: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.lines().collect();

        if lines.len() < 2 {
            return Ok(None); // Process not found
        }

        let line = lines[1]; // Skip header
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 12 {
            let pid = safe_parse_u32(parts[0]);
            let user = parts[2].to_string();
            let cpu_usage = safe_parse_f32(parts[3]);
            let memory_usage_percent = safe_parse_f32(parts[4]);
            let _virtual_memory = safe_parse_u64(parts[5]);
            let physical_memory = safe_parse_u64(parts[6]) * 1024; // Convert KB to bytes
            let status = parts[7].to_string();
            let etime = parts[8].to_string();
            let name = parts[9].to_string();
            let command = parts[10..].join(" ");
            let priority = safe_parse_i32(parts[11]);

            let start_time = parse_etime(&etime)?;
            let memory_usage = physical_memory;

            Ok(Some(ProcessInfo {
                pid,
                name,
                command,
                cpu_usage,
                memory_usage,
                memory_usage_percent,
                status,
                start_time,
                user,
                priority,
            }))
        } else {
            Ok(None)
        }
    }

    // Helper methods for collecting specific system information

    fn get_hostname(&self) -> Result<String> {
        Command::new("hostname")
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
            .or_else(|_| env::var("HOSTNAME"))
            .map_err(|_| anyhow!("Failed to get hostname"))
    }

    fn get_os_info(&self) -> Result<(String, String)> {
        let output = Command::new("cat")
            .args(&["/etc/os-release"])
            .output()
            .map_err(|_| anyhow!("Failed to read /etc/os-release"))?;

        let content = String::from_utf8_lossy(&output.stdout);
        let mut os_name = env::consts::OS.to_string();
        let mut os_version = "Unknown".to_string();

        for line in content.lines() {
            if line.starts_with("NAME=") {
                os_name = line
                    .split('=')
                    .nth(1)
                    .unwrap_or("Unknown")
                    .trim_matches('"')
                    .to_string();
            } else if line.starts_with("VERSION=") {
                os_version = line
                    .split('=')
                    .nth(1)
                    .unwrap_or("Unknown")
                    .trim_matches('"')
                    .to_string();
            }
        }

        Ok((os_name, os_version))
    }

    fn get_kernel_version(&self) -> Result<String> {
        Command::new("uname")
            .arg("-r")
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
            .map_err(|_| anyhow!("Failed to get kernel version"))
    }

    fn get_uptime(&self) -> Result<u64> {
        Command::new("cat")
            .arg("/proc/uptime")
            .output()
            .map(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0) as u64
            })
            .map_err(|_| anyhow!("Failed to read /proc/uptime"))
    }

    fn get_cpu_model(&self) -> Result<String> {
        Command::new("cat")
            .arg("/proc/cpuinfo")
            .output()
            .map(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .find(|line| line.starts_with("model name"))
                    .and_then(|line| line.split(':').nth(1))
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| "Unknown CPU".to_string())
            })
            .map_err(|_| anyhow!("Failed to read /proc/cpuinfo"))
    }

    fn get_cpu_frequency(&self) -> Result<u64> {
        Command::new("cat")
            .arg("/proc/cpuinfo")
            .output()
            .map(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .find(|line| line.starts_with("cpu MHz"))
                    .and_then(|line| line.split(':').nth(1))
                    .and_then(|s| s.trim().parse::<f64>().ok())
                    .unwrap_or(0.0) as u64
            })
            .map_err(|_| anyhow!("Failed to read CPU frequency"))
    }

    fn get_cpu_usage(&self) -> Result<f32> {
        let output = Command::new("cat")
            .arg("/proc/stat")
            .output()
            .map_err(|_| anyhow!("Failed to read /proc/stat"))?;

        let content = String::from_utf8_lossy(&output.stdout);
        let cpu_line = content
            .lines()
            .find(|line| line.starts_with("cpu "))
            .ok_or_else(|| anyhow!("CPU line not found in /proc/stat"))?;

        let parts: Vec<&str> = cpu_line.split_whitespace().collect();
        if parts.len() < 5 {
            return Err(anyhow!("Invalid CPU line format"));
        }

        let user = safe_parse_u64(parts[1]);
        let nice = safe_parse_u64(parts[2]);
        let system = safe_parse_u64(parts[3]);
        let idle = safe_parse_u64(parts[4]);

        let total = user + nice + system + idle;
        let used = total - idle;

        if total == 0 {
            Ok(0.0)
        } else {
            Ok((used as f32 / total as f32) * 100.0)
        }
    }

    fn get_cpu_temperature(&self) -> Option<f32> {
        // Try different temperature file locations
        let temp_files = [
            "/sys/class/thermal/thermal_zone0/temp",
            "/proc/acpi/thermal_zone/THM0/temperature",
            "/sys/class/hwmon/hwmon0/temp1_input",
        ];

        for temp_file in &temp_files {
            if let Ok(output) = Command::new("cat").arg(temp_file).output() {
                if let Ok(temp_str) = String::from_utf8(output.stdout) {
                    if let Ok(temp) = temp_str.trim().parse::<f32>() {
                        // Convert from millidegrees to degrees Celsius
                        return Some(temp / 1000.0);
                    }
                }
            }
        }

        None
    }

    fn read_proc_meminfo(&self) -> Result<String> {
        Command::new("cat")
            .arg("/proc/meminfo")
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
            .map_err(|e| anyhow!("Failed to read /proc/meminfo: {}", e))
    }

    fn get_interface_ip(&self, interface: &str) -> Result<String> {
        let output = Command::new("ip")
            .args(&["addr", "show", interface])
            .output()
            .map_err(|e| anyhow!("Failed to get IP for interface {}: {}", interface, e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);

        for line in output_str.lines() {
            if line.contains("inet ") && !line.contains("inet6") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return Ok(parts[1].split('/').next().unwrap_or("N/A").to_string());
                }
            }
        }

        Ok("N/A".to_string())
    }

    fn get_interface_mac(&self, interface: &str) -> Result<String> {
        let output = Command::new("ip")
            .args(&["link", "show", interface])
            .output()
            .map_err(|e| anyhow!("Failed to get MAC for interface {}: {}", interface, e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);

        for line in output_str.lines() {
            if line.contains("link/ether") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return Ok(parts[1].to_string());
                }
            }
        }

        Ok("N/A".to_string())
    }
}
