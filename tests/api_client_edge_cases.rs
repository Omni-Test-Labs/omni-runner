use omni_runner::api::client::ApiClient;
use omni_runner::models::{Heartbeat, RunnerStatus, DeviceBinding, TaskManifest, PipelineStep};
use std::collections::HashMap;

#[test]
fn test_api_client_edge_empty_device_id() {
    let result = ApiClient::new("http://test.com".to_string(), "".to_string(), None);
    assert!(result.is_ok());
}

#[test]
fn test_api_client_edge_whitespace_device_id() {
    let result = ApiClient::new("http://test.com".to_string(), "   ".to_string(), None);
    assert!(result.is_ok());
}

#[test]
fn test_api_client_edge_invalid_url() {
    let result = ApiClient::new("invalid-url".to_string(), "device-001".to_string(), None);
    assert!(result.is_ok());
}

#[test]
fn test_api_client_edge_ipv4_url() {
    let result = ApiClient::new("http://192.168.1.1:8080".to_string(), "device-001".to_string(), None);
    assert!(result.is_ok());
}

#[test]
fn test_api_client_edge_ipv6_url() {
    let result = ApiClient::new("http://[::1]:8080".to_string(), "device-001".to_string(), None);
    assert!(result.is_ok());
}

#[test]
fn test_api_client_edge_empty_api_key() {
    let result = ApiClient::new("http://test.com".to_string(), "device-001".to_string(), Some("".to_string()));
    assert!(result.is_ok());
}

#[test]
fn test_heartbeat_edge_zero_progress() {
    let sys_resources = omni_runner::models::SystemResources {
        cpu_percent: 0.0,
        memory_used_mb: 0,
        memory_total_mb: 8192,
        disk_used_gb: 0,
        disk_total_gb: 500,
    };
    
    let heartbeat = Heartbeat {
        device_id: "test".to_string(),
        runner_version: "0.1.0".to_string(),
        status: RunnerStatus::Idle,
        current_task_id: None,
        current_task_progress: 0.0,
        system_resources: sys_resources,
        capabilities: omni_runner::models::Capabilities {
            supported_step_types: vec![],
            has_oob_capture: false,
            has_gpu: false,
            gpu_model: None,
            oob_methods: vec![],
        },
        last_report: "2024-01-01".to_string(),
    };
    
    assert_eq!(heartbeat.current_task_progress, 0.0);
    assert!(heartbeat.capabilities.supported_step_types.is_empty());
}

#[test]
fn test_heartbeat_edge_max_progress() {
    let heartbeat = Heartbeat {
        device_id: "test".to_string(),
        runner_version: "0.1.0".to_string(),
        status: RunnerStatus::Running,
        current_task_id: Some("task-001".to_string()),
        current_task_progress: 100.0,
        system_resources: test_system_resources(),
        capabilities: test_capabilities(),
        last_report: "2024-01-01".to_string(),
    };
    
    assert_eq!(heartbeat.current_task_progress, 100.0);
    assert_eq!(heartbeat.status, RunnerStatus::Running);
}

#[test]
fn test_pipeline_step_edge_minimal() {
    let step = PipelineStep {
        step_id: "minimal-step".to_string(),
        order: 1,
        step_type: omni_runner::models::StepType::Shell,
        cmd: "echo test".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: false,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 0,
        artifact_collection: None,
    };
    
    assert_eq!(step.order, 1);
    assert_eq!(step.timeout_seconds, 0);
    assert!(step.depends_on.is_empty());
}

#[test]
fn test_device_binding_edge_empty_methods() {
    let binding = DeviceBinding {
        device_id: "test-device".to_string(),
        device_type: "test".to_string(),
        oob_methods: vec![],
    };
    
    assert!(binding.oob_methods.is_empty());
}

fn test_system_resources() -> omni_runner::models::SystemResources {
    omni_runner::models::SystemResources {
        cpu_percent: 50.0,
        memory_used_mb: 4096,
        memory_total_mb: 8192,
        disk_used_gb: 100,
        disk_total_gb: 500,
    }
}

fn test_capabilities() -> omni_runner::models::Capabilities {
    omni_runner::models::Capabilities {
        supported_step_types: vec![
            omni_runner::models::StepType::Shell,
        ],
        has_oob_capture: false,
        has_gpu: false,
        gpu_model: None,
        oob_methods: vec![],
    }
}
