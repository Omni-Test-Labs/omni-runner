use std::collections::HashMap;

use omni_runner::models::*;
use omni_runner::executor::*;
use omni_runner::executor::Executor as _;

#[test]
fn test_shell_executor_default() {
    let executor = ShellExecutor::new();
    assert_eq!(format!("{:?}", executor), "ShellExecutor");
}

#[test]
fn test_python_executor_default() {
    let executor = PythonExecutor::new();
    assert_eq!(format!("{:?}", executor), "PythonExecutor");
}

#[tokio::test]
async fn test_shell_executor_simple_command() {
    let executor = ShellExecutor::new();
    let step = PipelineStep {
        step_id: "test-step".to_string(),
        order: 1,
        step_type: StepType::Shell,
        cmd: "echo hello".to_string(),
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

    assert_eq!(result.step_id, "test-step");
    assert_eq!(result.status, TaskStatus::Success);
    assert_eq!(result.exit_code, Some(0));
    assert!(result.stdout_lines.is_some());
}

#[tokio::test]
async fn test_shell_executor_timeout() {
    let executor = ShellExecutor::new();
    let step = PipelineStep {
        step_id: "test-timeout".to_string(),
        order: 1,
        step_type: StepType::Shell,
        cmd: "sleep 10".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: SecurityPolicy::default(),
        timeout_seconds: 1,
        artifact_collection: None,
    };

    let result: StepResult = executor.execute(&step).await.unwrap();

    assert_eq!(result.status, TaskStatus::Timeout);
    assert_eq!(result.signal, Some("SIGKILL".to_string()));
}

#[tokio::test]
async fn test_shell_executor_with_env() {
    let executor = ShellExecutor::new();
    let mut env = HashMap::new();
    env.insert("MY_VAR".to_string(), "test_value".to_string());

    let step = PipelineStep {
        step_id: "test-env".to_string(),
        order: 1,
        step_type: StepType::Shell,
        cmd: "echo $MY_VAR".to_string(),
        env,
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
    assert_eq!(result.status, TaskStatus::Success);
}

#[tokio::test]
async fn test_shell_executor_command_fails() {
    let executor = ShellExecutor::new();
    let step = PipelineStep {
        step_id: "test-fail".to_string(),
        order: 1,
        step_type: StepType::Shell,
        cmd: "/bin/sh -c 'exit 1'".to_string(),
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

    assert_ne!(result.status, TaskStatus::Success);
    assert!(result.exit_code.is_some());
    assert_eq!(result.exit_code.unwrap_or(0), 1);
}
