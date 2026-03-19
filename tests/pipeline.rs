use std::collections::HashMap;

use omni_runner::pipeline::PipelineEngine;
use omni_runner::models::*;

#[tokio::test]
async fn test_pipeline_engine_simple_execution() {
    let engine = PipelineEngine::new();
    let manifest = TaskManifest {
        schema_version: "1.0.0".to_string(),
        task_id: "task-123".to_string(),
        created_at: "2024-03-18T10:00:00Z".to_string(),
        device_binding: DeviceBinding {
            device_id: "device-001".to_string(),
            device_type: "pc".to_string(),
            oob_methods: vec![],
        },
        priority: Priority::Normal,
        timeout_seconds: 300,
        pipeline: vec![
            PipelineStep {
                step_id: "step-1".to_string(),
                order: 1,
                step_type: StepType::Shell,
                cmd: "echo 'step 1'".to_string(),
                env: HashMap::new(),
                working_dir: None,
                must_pass: true,
                depends_on: vec![],
                always_run: false,
                retry_policy: None,
                security_policy: SecurityPolicy::default(),
                timeout_seconds: 10,
                artifact_collection: None,
            },
            PipelineStep {
                step_id: "step-2".to_string(),
                order: 2,
                step_type: StepType::Shell,
                cmd: "echo 'step 2'".to_string(),
                env: HashMap::new(),
                working_dir: None,
                must_pass: true,
                depends_on: vec![],
                always_run: false,
                retry_policy: None,
                security_policy: SecurityPolicy::default(),
                timeout_seconds: 10,
                artifact_collection: None,
            },
        ],
        notification_hooks: None,
    };

    let results = engine.execute(&manifest).await.unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].step_id, "step-1");
    assert_eq!(results[1].step_id, "step-2");
    assert_eq!(results[0].status, TaskStatus::Success);
    assert_eq!(results[1].status, TaskStatus::Success);
}

#[tokio::test]
async fn test_pipeline_engine_with_dependencies() {
    let engine = PipelineEngine::new();
    let manifest = TaskManifest {
        schema_version: "1.0.0".to_string(),
        task_id: "task-456".to_string(),
        created_at: "2024-03-18T10:00:00Z".to_string(),
        device_binding: DeviceBinding {
            device_id: "device-001".to_string(),
            device_type: "pc".to_string(),
            oob_methods: vec![],
        },
        priority: Priority::Normal,
        timeout_seconds: 300,
        pipeline: vec![
            PipelineStep {
                step_id: "step-1".to_string(),
                order: 1,
                step_type: StepType::Shell,
                cmd: "echo 'step 1'".to_string(),
                env: HashMap::new(),
                working_dir: None,
                must_pass: true,
                depends_on: vec![],
                always_run: false,
                retry_policy: None,
                security_policy: SecurityPolicy::default(),
                timeout_seconds: 10,
                artifact_collection: None,
            },
            PipelineStep {
                step_id: "step-2".to_string(),
                order: 2,
                step_type: StepType::Shell,
                cmd: "echo 'step 2'".to_string(),
                env: HashMap::new(),
                working_dir: None,
                must_pass: true,
                depends_on: vec!["step-1".to_string()],
                always_run: false,
                retry_policy: None,
                security_policy: SecurityPolicy::default(),
                timeout_seconds: 10,
                artifact_collection: None,
            },
        ],
        notification_hooks: None,
    };

    let results = engine.execute(&manifest).await.unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].step_id, "step-1");
    assert_eq!(results[1].step_id, "step-2");
    assert_eq!(results[0].status, TaskStatus::Success);
    assert_eq!(results[1].status, TaskStatus::Success);
}

#[tokio::test]
async fn test_pipeline_engine_dependency_failure_skips() {
    let engine = PipelineEngine::new();
    let manifest = TaskManifest {
        schema_version: "1.0.0".to_string(),
        task_id: "task-789".to_string(),
        created_at: "2024-03-18T10:00:00Z".to_string(),
        device_binding: DeviceBinding {
            device_id: "device-001".to_string(),
            device_type: "pc".to_string(),
            oob_methods: vec![],
        },
        priority: Priority::Normal,
        timeout_seconds: 300,
        pipeline: vec![
            PipelineStep {
                step_id: "step-1".to_string(),
                order: 1,
                step_type: StepType::Shell,
                cmd: "false".to_string(),
                env: HashMap::new(),
                working_dir: None,
                must_pass: true,
                depends_on: vec![],
                always_run: false,
                retry_policy: None,
                security_policy: SecurityPolicy::default(),
                timeout_seconds: 10,
                artifact_collection: None,
            },
            PipelineStep {
                step_id: "step-2".to_string(),
                order: 2,
                step_type: StepType::Shell,
                cmd: "echo 'step 2'".to_string(),
                env: HashMap::new(),
                working_dir: None,
                must_pass: true,
                depends_on: vec!["step-1".to_string()],
                always_run: false,
                retry_policy: None,
                security_policy: SecurityPolicy::default(),
                timeout_seconds: 10,
                artifact_collection: None,
            },
        ],
        notification_hooks: None,
    };

    let results = engine.execute(&manifest).await.unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].step_id, "step-1");
    assert_eq!(results[1].step_id, "step-2");
    assert_ne!(results[0].status, TaskStatus::Success);
    assert_eq!(results[1].status, TaskStatus::Skipped);
    assert!(results[1].reason.is_some());
}

#[tokio::test]
async fn test_pipeline_engine_must_pass_stops_on_failure() {
    let engine = PipelineEngine::new();
    let manifest = TaskManifest {
        schema_version: "1.0.0".to_string(),
        task_id: "task-999".to_string(),
        created_at: "2024-03-18T10:00:00Z".to_string(),
        device_binding: DeviceBinding {
            device_id: "device-001".to_string(),
            device_type: "pc".to_string(),
            oob_methods: vec![],
        },
        priority: Priority::Normal,
        timeout_seconds: 300,
        pipeline: vec![
            PipelineStep {
                step_id: "step-1".to_string(),
                order: 1,
                step_type: StepType::Shell,
                cmd: "echo 'step 1'".to_string(),
                env: HashMap::new(),
                working_dir: None,
                must_pass: false,
                depends_on: vec![],
                always_run: false,
                retry_policy: None,
                security_policy: SecurityPolicy::default(),
                timeout_seconds: 10,
                artifact_collection: None,
            },
            PipelineStep {
                step_id: "step-2".to_string(),
                order: 2,
                step_type: StepType::Shell,
                cmd: "false".to_string(),
                env: HashMap::new(),
                working_dir: None,
                must_pass: true,
                depends_on: vec![],
                always_run: false,
                retry_policy: None,
                security_policy: SecurityPolicy::default(),
                timeout_seconds: 10,
                artifact_collection: None,
            },
            PipelineStep {
                step_id: "step-3".to_string(),
                order: 3,
                step_type: StepType::Shell,
                cmd: "echo 'step 3'".to_string(),
                env: HashMap::new(),
                working_dir: None,
                must_pass: true,
                depends_on: vec![],
                always_run: false,
                retry_policy: None,
                security_policy: SecurityPolicy::default(),
                timeout_seconds: 10,
                artifact_collection: None,
            },
        ],
        notification_hooks: None,
    };

    let results = engine.execute(&manifest).await.unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].step_id, "step-1");
    assert_eq!(results[1].step_id, "step-2");
    assert_eq!(results[0].status, TaskStatus::Success);
    assert_ne!(results[1].status, TaskStatus::Success);
}
