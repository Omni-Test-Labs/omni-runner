use std::collections::HashMap;

use omni_runner::executor::*;
use omni_runner::models::*;

#[test]
fn test_binary_executor_default() {
    let executor = BinaryExecutor::new();
    assert_eq!(format!("{:?}", executor), "BinaryExecutor");
}

#[tokio::test]
async fn test_binary_executor_simple_command() {
    let executor = BinaryExecutor::new();
    let step = PipelineStep {
        step_id: "test-binary".to_string(),
        order: 1,
        step_type: StepType::Binary,
        cmd: "echo".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: SecurityPolicy::default(),
        timeout_seconds: 10,
        artifact_collection: None,
    };

    let result: StepResult = executor.execute(&step).await.unwrap();

    assert_eq!(result.step_id, "test-binary");
    assert_eq!(result.status, TaskStatus::Success);
    assert_eq!(result.exit_code, Some(0));
}

#[tokio::test]
#[ignore]
async fn test_binary_executor_with_args() {
    let executor = BinaryExecutor::new();
    let step = PipelineStep {
        step_id: "test-binary-args".to_string(),
        order: 1,
        step_type: StepType::Binary,
        cmd: "echo hello world".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: SecurityPolicy::default(),
        timeout_seconds: 10,
        artifact_collection: None,
    };

    let result: StepResult = executor.execute(&step).await.unwrap();

    assert_eq!(result.step_id, "test-binary-args");
    assert_eq!(result.status, TaskStatus::Success);
    assert_eq!(result.exit_code, Some(0));
}

#[tokio::test]
async fn test_binary_executor_command_fails() {
    let executor = BinaryExecutor::new();
    let step = PipelineStep {
        step_id: "test-binary-fail".to_string(),
        order: 1,
        step_type: StepType::Binary,
        cmd: "nonexistent_binary_12345".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: SecurityPolicy::default(),
        timeout_seconds: 10,
        artifact_collection: None,
    };

    // Should fail with non-existent binary
    let result = executor.execute(&step).await;

    assert!(result.is_err());
}
