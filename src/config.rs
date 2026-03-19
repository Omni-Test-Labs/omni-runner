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
    #[serde(default = "default_interval_seconds")]
    pub interval_seconds: u64,

    /// Interval between heartbeat reports (seconds)
    #[serde(default = "default_heartbeat_interval_seconds")]
    pub heartbeat_interval_seconds: u64,
}

/// Execution workspace and logging configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ExecutionConfig {
    /// Base directory for task execution workspace
    #[serde(default = "default_workspace_dir")]
    pub workspace_dir: PathBuf,

    /// Base directory for task logs
    #[serde(default = "default_log_dir")]
    pub log_dir: PathBuf,
}

fn default_interval_seconds() -> u64 { 5 }
fn default_heartbeat_interval_seconds() -> u64 { 30 }
fn default_workspace_dir() -> PathBuf { PathBuf::from("./workspace") }
fn default_log_dir() -> PathBuf { PathBuf::from("./logs") }

/// Complete omni-runner configuration
#[derive(Debug, Deserialize)]
pub struct RunnerConfig {
    pub server: ServerConfig,
    pub device: DeviceConfig,
    #[serde(default)]
    pub polling: PollingConfig,
    #[serde(default)]
    pub execution: ExecutionConfig,
}

/// Load configuration from a TOML file
pub fn load_config(config_path: &str) -> Result<RunnerConfig> {
    use config::FileFormat;

    let path = std::path::Path::new(config_path);
    let config = config::Config::builder()
        .add_source(
            config::File::from(path)
                .format(FileFormat::Toml)
                .required(true)
        )
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
