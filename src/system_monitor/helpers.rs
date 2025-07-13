//! Helper Functions for System Monitoring
//!
//! This module contains utility functions for parsing system data,
//! converting between formats, and other helper operations.

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};

/// Parses a size string (e.g., "1K", "2M", "3G") into bytes
///
/// # Arguments
///
/// * `size_str` - The size string to parse
///
/// # Returns
///
/// Returns the size in bytes
///
/// # Errors
///
/// Returns an error if the size string cannot be parsed
pub fn parse_size(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim();

    if size_str.is_empty() {
        return Ok(0);
    }

    // Try to parse as a number first
    if let Ok(size) = size_str.parse::<u64>() {
        return Ok(size);
    }

    // Parse with unit suffix
    let (number_str, unit) = size_str.split_at(size_str.len() - 1);
    let number = number_str
        .parse::<f64>()
        .map_err(|_| anyhow!("Invalid number in size string: {}", size_str))?;

    let multiplier = match unit.to_uppercase().as_str() {
        "K" | "KB" => 1024.0,
        "M" | "MB" => 1024.0 * 1024.0,
        "G" | "GB" => 1024.0 * 1024.0 * 1024.0,
        "T" | "TB" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => return Err(anyhow!("Unknown size unit: {}", unit)),
    };

    Ok((number * multiplier) as u64)
}

/// Parses elapsed time string from ps command into DateTime
///
/// # Arguments
///
/// * `etime` - The elapsed time string (e.g., "1-02:30:45")
///
/// # Returns
///
/// Returns the start time as DateTime<Utc>
///
/// # Errors
///
/// Returns an error if the elapsed time cannot be parsed
pub fn parse_etime(etime: &str) -> Result<DateTime<Utc>> {
    let now = Utc::now();

    if etime == "-" || etime.is_empty() {
        return Ok(now);
    }

    let parts: Vec<&str> = etime.split(':').collect();
    if parts.len() != 3 {
        return Err(anyhow!("Invalid elapsed time format: {}", etime));
    }

    let mut total_seconds = 0i64;

    // Parse days if present (format: "1-02:30:45")
    let time_part = parts[0];
    if time_part.contains('-') {
        let day_parts: Vec<&str> = time_part.split('-').collect();
        if day_parts.len() == 2 {
            let days: i64 = day_parts[0]
                .parse()
                .map_err(|_| anyhow!("Invalid days in elapsed time: {}", etime))?;
            total_seconds += days * 24 * 3600;

            let hours: i64 = day_parts[1]
                .parse()
                .map_err(|_| anyhow!("Invalid hours in elapsed time: {}", etime))?;
            total_seconds += hours * 3600;
        } else {
            return Err(anyhow!("Invalid elapsed time format: {}", etime));
        }
    } else {
        let hours: i64 = time_part
            .parse()
            .map_err(|_| anyhow!("Invalid hours in elapsed time: {}", etime))?;
        total_seconds += hours * 3600;
    }

    // Parse minutes
    let minutes: i64 = parts[1]
        .parse()
        .map_err(|_| anyhow!("Invalid minutes in elapsed time: {}", etime))?;
    total_seconds += minutes * 60;

    // Parse seconds
    let seconds: i64 = parts[2]
        .parse()
        .map_err(|_| anyhow!("Invalid seconds in elapsed time: {}", etime))?;
    total_seconds += seconds;

    Ok(now - chrono::Duration::seconds(total_seconds))
}

/// Safely parses a string to u32, returning 0 if parsing fails
///
/// # Arguments
///
/// * `s` - The string to parse
///
/// # Returns
///
/// Returns the parsed u32 value or 0 if parsing fails
pub fn safe_parse_u32(s: &str) -> u32 {
    s.trim().parse().unwrap_or(0)
}

/// Safely parses a string to f32, returning 0.0 if parsing fails
///
/// # Arguments
///
/// * `s` - The string to parse
///
/// # Returns
///
/// Returns the parsed f32 value or 0.0 if parsing fails
pub fn safe_parse_f32(s: &str) -> f32 {
    s.trim().parse().unwrap_or(0.0)
}

/// Safely parses a string to u64, returning 0 if parsing fails
///
/// # Arguments
///
/// * `s` - The string to parse
///
/// # Returns
///
/// Returns the parsed u64 value or 0 if parsing fails
pub fn safe_parse_u64(s: &str) -> u64 {
    s.trim().parse().unwrap_or(0)
}

/// Safely parses a string to i32, returning 0 if parsing fails
///
/// # Arguments
///
/// * `s` - The string to parse
///
/// # Returns
///
/// Returns the parsed i32 value or 0 if parsing fails
pub fn safe_parse_i32(s: &str) -> i32 {
    s.trim().parse().unwrap_or(0)
}

/// Extracts a value from a key-value line (e.g., "MemTotal: 16384 kB")
///
/// # Arguments
///
/// * `line` - The line to parse
/// * `key` - The key to look for
///
/// # Returns
///
/// Returns the value as a string if found, None otherwise
pub fn extract_value(line: &str, key: &str) -> Option<String> {
    if line.starts_with(key) {
        line.split(':').nth(1).map(|s| s.trim().to_string())
    } else {
        None
    }
}

/// Extracts a numeric value from a key-value line and converts it to u64
///
/// # Arguments
///
/// * `line` - The line to parse
/// * `key` - The key to look for
///
/// # Returns
///
/// Returns the value as u64 if found and valid, None otherwise
pub fn extract_numeric_value(line: &str, key: &str) -> Option<u64> {
    extract_value(line, key)
        .and_then(|v| v.split_whitespace().next().map(|s| s.to_string()))
        .and_then(|v| v.parse::<u64>().ok())
}

/// Formats bytes into a human-readable string
///
/// # Arguments
///
/// * `bytes` - The number of bytes
///
/// # Returns
///
/// Returns a formatted string (e.g., "1.5 GB")
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Calculates percentage with safe division
///
/// # Arguments
///
/// * `part` - The part value
/// * `total` - The total value
///
/// # Returns
///
/// Returns the percentage (0.0 - 100.0) or 0.0 if total is zero
pub fn calculate_percentage(part: u64, total: u64) -> f32 {
    if total == 0 {
        0.0
    } else {
        (part as f32 / total as f32) * 100.0
    }
}
