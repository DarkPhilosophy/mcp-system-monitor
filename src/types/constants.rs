//! MCP Protocol Constants
//!
//! Constant values used in MCP protocol including method names and error codes.

/// MCP Method Names
///
/// Available methods that can be called via MCP protocol.

/// Get system information (hostname, OS, kernel, uptime)
pub const METHOD_GET_SYSTEM_INFO: &str = "getSystemInfo";

/// Get CPU information (usage, frequency, cores, temperature)
pub const METHOD_GET_CPU_INFO: &str = "getCPUInfo";

/// Get memory information (RAM and swap usage)
pub const METHOD_GET_MEMORY_INFO: &str = "getMemoryInfo";

/// Get disk information (storage usage, filesystems)
pub const METHOD_GET_DISK_INFO: &str = "getDiskInfo";

/// Get network information (interfaces, traffic statistics)
pub const METHOD_GET_NETWORK_INFO: &str = "getNetworkInfo";

/// Get all running processes
pub const METHOD_GET_PROCESSES: &str = "getProcesses";

/// Get specific process by PID
pub const METHOD_GET_PROCESS_BY_PID: &str = "getProcessByPID";

/// Get complete system metrics snapshot
pub const METHOD_GET_SYSTEM_METRICS: &str = "getSystemMetrics";

/// Start continuous monitoring
pub const METHOD_START_MONITORING: &str = "startMonitoring";

/// Stop continuous monitoring
pub const METHOD_STOP_MONITORING: &str = "stopMonitoring";

/// Error Codes
///
/// Error codes used in MCP error responses following JSON-RPC 2.0 specification.

/// Invalid Request - The JSON sent is not a valid Request object
pub const ERROR_INVALID_REQUEST: i32 = -32600;

/// Method not found - The method does not exist / is not available
pub const ERROR_METHOD_NOT_FOUND: i32 = -32601;

/// Invalid params - Invalid method parameter(s)
pub const ERROR_INVALID_PARAMS: i32 = -32602;

/// Internal error - Internal JSON-RPC error
pub const ERROR_INTERNAL_ERROR: i32 = -32603;

/// Parse error - Invalid JSON was received by the server
pub const ERROR_PARSE_ERROR: i32 = -32700;

/// Process not found - The specified process PID does not exist
pub const ERROR_PROCESS_NOT_FOUND: i32 = -32001;

/// Monitoring already started - Continuous monitoring is already active
pub const ERROR_MONITORING_ALREADY_STARTED: i32 = -32002;

/// Monitoring not started - Continuous monitoring is not active
pub const ERROR_MONITORING_NOT_STARTED: i32 = -32003;

/// System command failed - A required system command failed to execute
pub const ERROR_SYSTEM_COMMAND_FAILED: i32 = -32004;

/// Permission denied - Insufficient permissions to access system information
pub const ERROR_PERMISSION_DENIED: i32 = -32005;


