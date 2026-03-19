use omni_runner::heartbeat::create_heartbeat;
use omni_runner::models::{Heartbeat, RunnerStatus, StepType};

#[test]
fn test_create_heartbeat_has_device_id() {
    let heartbeat = create_heartbeat("test-device-001").unwrap();
    assert_eq!(heartbeat.device_id, "test-device-001");
}

#[test]
fn test_create_heartbeat_has_valid_version() {
    let heartbeat = create_heartbeat("test-device").unwrap();
    assert_eq!(heartbeat.runner_version, "0.1.0");
}

#[test]
fn test_create_heartbeat_idle_status() {
    let heartbeat = create_heartbeat("test-device").unwrap();
    assert_eq!(heartbeat.status, RunnerStatus::Idle);
}

#[test]
fn test_create_heartbeat_no_current_task() {
    let heartbeat = create_heartbeat("test-device").unwrap();
    assert!(heartbeat.current_task_id.is_none(), "Current task should be None");
}

#[test]
fn test_create_heartbeat_zero_progress() {
    let heartbeat = create_heartbeat("test-device").unwrap();
    assert_eq!(heartbeat.current_task_progress, 0.0);
}

#[test]
fn test_create_heartbeat_cpu_resources_valid() {
    let heartbeat = create_heartbeat("test-device").unwrap();
    assert!(heartbeat.system_resources.cpu_percent >= 0.0);
    assert!(heartbeat.system_resources.cpu_percent <= 100.0);
}

#[test]
fn test_create_heartbeat_memory_resources_valid() {
    let heartbeat = create_heartbeat("test-device").unwrap();
    assert!(heartbeat.system_resources.memory_total_mb > 0);
    assert!(heartbeat.system_resources.memory_used_mb >= 0);
}

#[test]
fn test_create_heartbeat_disk_resources_valid() {
    let heartbeat = create_heartbeat("test-device").unwrap();
    assert!(heartbeat.system_resources.disk_used_gb > 0);
    assert!(heartbeat.system_resources.disk_total_gb > 0);
}

#[test]
fn test_create_heartbeat_has_capabilities() {
    let heartbeat = create_heartbeat("test-device").unwrap();
    assert!(!heartbeat.capabilities.supported_step_types.is_empty(),
            "Should have supported step types");
}

#[test]
fn test_create_heartbeat_all_step_types() {
    let heartbeat = create_heartbeat("test-device").unwrap();
    let expected_types = vec![StepType::Python, StepType::Binary, StepType::Shell, StepType::Api];
    assert_eq!(heartbeat.capabilities.supported_step_types, expected_types);
}

#[test]
fn test_create_heartbeat_no_gpu() {
    let heartbeat = create_heartbeat("test-device").unwrap();
    assert!(!heartbeat.capabilities.has_gpu);
    assert!(heartbeat.capabilities.gpu_model.is_none());
}

#[test]
fn test_create_heartbeat_default_capabilities() {
    let heartbeat = create_heartbeat("test-device").unwrap();
    assert!(!heartbeat.capabilities.has_oob_capture);
    assert!(heartbeat.capabilities.oob_methods.is_empty());
}

#[test]
fn test_create_heartbeat_has_timestamp() {
    let heartbeat = create_heartbeat("test-device").unwrap();
    assert!(!heartbeat.last_report.is_empty());
}
