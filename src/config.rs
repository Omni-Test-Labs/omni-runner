use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

/// Server configuration for omni-server API endpoints
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    /// Base URL of the omni-server API
    pub base_url: String,
    
    /// API key for authentication
    #[serde(default)]
    pub api_key: Option<String>,
}

/// Device identification and type configuration
#[derive(Debug, Deserialize, Clone)]
pub struct DeviceConfig {
    /// Unique device identifier (UUID)
    pub device_id: String,
    
    /// Device type (pc, mobile, iot, server)
    pub device_type: String,
    
    /// Device hostname
    pub hostname: String,
}

/// Polling interval configuration
#[derive(Debug, Deserialize, Clone)]
pub struct PollingConfig {
    /// Interval between task polling attempts (seconds)
    #[serde(default = "5")]
    pub interval_seconds: u64,
    
    /// Interval between heartbeat reports (seconds)
    #[serde(default = "30")]
    pub heartbeat_interval_seconds: u64,
}

/// Execution workspace and logging configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ExecutionConfig {
    /// Base directory for task execution workspace
    #[serde(default = "./workspace")]
    pub workspace_dir: PathBuf,
    
    /// Base directory for task logs
    #[serde(default = "./logs")]
    pub log_dir: PathBuf,
}

/// Complete omni-runner configuration
#[derive(Debug, Deserialize)]
pub struct RunnerConfig {
    pub server: ServerConfig,
    pub device: DeviceConfig,
    pub polling: PollingConfig,
    pub execution: ExecutionConfig,
}

/// Load configuration from a TOML file
pub fn load_config(config_path: &str) -> Result<RunnerConfig> {
    let config = config::Config::builder()
        .add_source(config::File::with_name(config_path).required(true))
        .build()
        .with_context(|| format!("Failed to load configuration from {}", config_path))?;

    config.try_deserialize()
        .with_context(|| "Failed to deserialize configuration")
}

impl Default for PollingConfig {
    fn default() -> Self {
        Self {
            interval_seconds: 5,
            heartbeat_interval_seconds: 30,
        }
    }
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            workspace_dir: PathBuf::from("./workspace"),
            log_dir: PathBuf::from("./logs"),
        }
    }
}
