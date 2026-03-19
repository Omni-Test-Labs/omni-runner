use std::collections::HashMap;

use omni_runner::models::*;

#[test]
fn test_task_status_serialization() {
    let status = TaskStatus::Success;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, r#""success""#);

    let deserialized: TaskStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, TaskStatus::Success);
}

#[test]
fn test_step_type_serialization() {
    let step_type = StepType::Shell;
    let json = serde_json::to_string(&step_type).unwrap();
    assert_eq!(json, r#""shell""#);

    let deserialized: StepType = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, StepType::Shell);
}

#[test]
fn test_pipeline_step_serialization() {
    let step = PipelineStep {
        step_id: "test-step".to_string(),
        order: 1,
        step_type: StepType::Shell,
        cmd: "echo hello".to_string(),
        env: HashMap::new(),
        working_dir: Some("/tmp".to_string()),
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: Some(RetryPolicy {
            max_retries: 3,
            retry_delay_seconds: 5,
            backoff_multiplier: 2.0,
        }),
        security_policy: SecurityPolicy {
            allow_sudo: false,
            forbidden_cmds: vec![],
            allowed_dirs: vec![],
            network_policy: NetworkPolicy {
                allow_internet: false,
                allowed_hosts: vec![],
            },
        },
        timeout_seconds: 30,
        artifact_collection: None,
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""type":"shell""#));
    assert!(json.contains(r#""cmd":"echo hello""#));

    let deserialized: PipelineStep = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.step_id, "test-step");
    assert_eq!(deserialized.step_type, StepType::Shell);
    assert_eq!(deserialized.cmd, "echo hello");
}

#[test]
fn test_task_manifest_serialization() {
    let manifest = TaskManifest {
        schema_version: "1.0.0".to_string(),
        task_id: "task-123".to_string(),
        created_at: "2024-03-18T10:00:00Z".to_string(),
        device_binding: DeviceBinding {
            device_id: "device-001".to_string(),
            device_type: "pc".to_string(),
            oob_methods: vec![],
        },
        priority: Priority::High,
        timeout_seconds: 300,
        pipeline: vec![],
        notification_hooks: None,
    };

    let json = serde_json::to_string(&manifest).unwrap();
    assert!(json.contains(r#""task_id":"task-123""#));
    assert!(json.contains(r#""priority":"high""#));

    let deserialized: TaskManifest = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.task_id, "task-123");
    assert_eq!(deserialized.priority, Priority::High);
}

#[test]
fn test_retry_policy_defaults() {
    let policy = RetryPolicy::default();
    assert_eq!(policy.max_retries, 0);
    assert_eq!(policy.retry_delay_seconds, 5);
    assert_eq!(policy.backoff_multiplier, 2.0);
}

#[test]
fn test_security_policy_defaults() {
    let policy = SecurityPolicy::default();
    assert_eq!(policy.allow_sudo, false);
    assert!(policy.forbidden_cmds.is_empty());
    assert!(policy.allowed_dirs.is_empty());
}
