use std::io::Write;
use tempfile::NamedTempFile;

use omni_runner::models::*;
use omni_runner::config::*;

#[test]
fn test_polling_config_defaults() {
    let config = PollingConfig::default();
    assert_eq!(config.interval_seconds, 5);
    assert_eq!(config.heartbeat_interval_seconds, 30);
}

#[test]
fn test_execution_config_defaults() {
    let config = ExecutionConfig::default();
    assert_eq!(config.workspace_dir, std::path::PathBuf::from("./workspace"));
    assert_eq!(config.log_dir, std::path::PathBuf::from("./logs"));
}

#[test]
fn test_load_config_from_toml() {
    let mut config_file = NamedTempFile::new().unwrap();

    writeln!(config_file, r#"
[server]
base_url = "http://localhost:8000"

[device]
device_id = "test-device-001"
device_type = "pc"
hostname = "test-host"

[polling]
interval_seconds = 10
heartbeat_interval_seconds = 60

[execution]
workspace_dir = "/tmp/workspace"
log_dir = "/tmp/logs"
"#).unwrap();
    config_file.flush().unwrap();

    let config = load_config(&config_file.path().to_str().unwrap()).unwrap();

    assert_eq!(config.server.base_url, "http://localhost:8000");
    assert_eq!(config.device.device_id, "test-device-001");
    assert_eq!(config.device.device_type, "pc");
    assert_eq!(config.device.hostname, "test-host");
    assert_eq!(config.polling.interval_seconds, 10);
    assert_eq!(config.polling.heartbeat_interval_seconds, 60);
    assert_eq!(config.execution.workspace_dir, std::path::PathBuf::from("/tmp/workspace"));
    assert_eq!(config.execution.log_dir, std::path::PathBuf::from("/tmp/logs"));
}

#[test]
fn test_load_config_with_defaults() {
    let mut config_file = NamedTempFile::new().unwrap();

    writeln!(config_file, r#"
[server]
base_url = "http://localhost:8000"

[device]
device_id = "test-device-001"
device_type = "pc"
hostname = "test-host"

[polling]
interval_seconds = 10
heartbeat_interval_seconds = 60

[execution]
workspace_dir = "/tmp/workspace"
log_dir = "/tmp/logs"
"#).unwrap();
    config_file.flush().unwrap();

    let config = load_config(&config_file.path().to_str().unwrap()).unwrap();

    assert_eq!(config.server.base_url, "http://localhost:8000");
    assert_eq!(config.device.device_id, "test-device-001");
    assert_eq!(config.polling.interval_seconds, 10);
    assert_eq!(config.polling.heartbeat_interval_seconds, 60);
    assert_eq!(config.execution.workspace_dir, std::path::PathBuf::from("/tmp/workspace"));
    assert_eq!(config.execution.log_dir, std::path::PathBuf::from("/tmp/logs"));
}
