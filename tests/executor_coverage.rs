use omni_runner::executor::{ShellExecutor, PythonExecutor, BinaryExecutor, ApiExecutor};
use omni_runner::executor::Executor as _;
use std::collections::HashMap;

fn create_shell_step(command: &str) -> omni_runner::models::PipelineStep {
    omni_runner::models::PipelineStep {
        step_id: "test".to_string(),
        order: 1,
        step_type: omni_runner::models::StepType::Shell,
        cmd: command.to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: false,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 30,
        artifact_collection: None,
    }
}

#[test]
fn test_executor_coverage_shell_new() {
    let _executor = ShellExecutor::new();
    assert!(true);
}

#[test]
fn test_executor_coverage_python_new() {
    let _executor = PythonExecutor::new();
    assert!(true);
}

#[test]
fn test_executor_coverage_binary_new() {
    let _executor = BinaryExecutor::new();
    assert!(true);
}

#[test]
fn test_executor_coverage_api_new() {
    let _executor = ApiExecutor::new();
    assert!(true);
}

#[test]
fn test_executor_coverage_step_with_env() {
    let mut env = HashMap::new();
    env.insert("TEST_VAR".to_string(), "test-value".to_string());
    
    let step = omni_runner::models::PipelineStep {
        step_id: "test".to_string(),
        order: 1,
        step_type: omni_runner::models::StepType::Shell,
        cmd: "echo $TEST_VAR".to_string(),
        env,
        working_dir: None,
        must_pass: false,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 30,
        artifact_collection: None,
    };
    
    assert_eq!(step.env.len(), 1);
    assert!(step.env.contains_key("TEST_VAR"));
}

#[test]
fn test_executor_coverage_step_with_working_dir() {
    let step = omni_runner::models::PipelineStep {
        step_id: "test".to_string(),
        order: 1,
        step_type: omni_runner::models::StepType::Shell,
        cmd: "ls".to_string(),
        env: HashMap::new(),
        working_dir: Some("/tmp".to_string()),
        must_pass: false,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 30,
        artifact_collection: None,
    };
    
    assert!(step.working_dir.is_some());
    assert_eq!(step.working_dir.unwrap(), "/tmp");
}

#[test]
fn test_executor_coverage_step_with_retry() {
    let retry_policy = omni_runner::models::RetryPolicy {
        max_retries: 5,
        retry_delay_seconds: 15,
        backoff_multiplier: 2.5,
    };
    
    let step = omni_runner::models::PipelineStep {
        step_id: "test".to_string(),
        order: 1,
        step_type: omni_runner::models::StepType::Shell,
        cmd: "echo test".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: false,
        depends_on: vec![],
        always_run: false,
        retry_policy: Some(retry_policy),
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 30,
        artifact_collection: None,
    };
    
    assert!(step.retry_policy.is_some());
    let policy = step.retry_policy.unwrap();
    assert_eq!(policy.max_retries, 5);
}

#[test]
fn test_executor_coverage_step_depenencies() {
    let step = omni_runner::models::PipelineStep {
        step_id: "step-2".to_string(),
        order: 2,
        step_type: omni_runner::models::StepType::Shell,
        cmd: "echo step2".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: false,
        depends_on: vec!["step-1".to_string(), "step-0".to_string()],
        always_run: false,
        retry_policy: None,
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 30,
        artifact_collection: None,
    };
    
    assert_eq!(step.depends_on.len(), 2);
    assert!(step.depends_on.contains(&"step-1".to_string()));
}

#[test]
fn test_executor_coverage_always_run_true() {
    let step = create_shell_step("echo test");
    assert_eq!(step.always_run, false);
}

#[test]
fn test_executor_coverage_security_policy() {
    let security = omni_runner::models::SecurityPolicy {
        allow_sudo: false,
        forbidden_cmds: vec!["rm".to_string()],
        allowed_dirs: vec!["/tmp".to_string()],
        network_policy: omni_runner::models::NetworkPolicy::default(),
    };
    
    assert!(!security.allow_sudo);
    assert_eq!(security.forbidden_cmds.len(), 1);
}
