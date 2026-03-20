use std::collections::HashMap;

use omni_runner::executor::*;
use omni_runner::models::{PipelineStep, StepType, SecurityPolicy, TaskStatus};

#[test]
fn test_api_executor_default() {
    let executor = ApiExecutor::new();
    assert_eq!(format!("{:?}", executor), "ApiExecutor");
}

#[tokio::test]
async fn test_api_executor_url_format() {
    let executor = ApiExecutor::new();
    let step = PipelineStep {
        step_id: "test-url-format".to_string(),
        order: 1,
        step_type: StepType::Api,
        cmd: "http://example.com/api/test".to_string(),
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

    assert_eq!(step.step_type, StepType::Api);
    assert!(!step.cmd.is_empty());
    let result = executor.execute(&step).await;
    assert!(result.is_ok());

    let step_result = result.unwrap();
    assert_eq!(step_result.step_id, "test-url-format");
    assert_eq!(step_result.status, TaskStatus::Success);
    assert!(step_result.started_at.is_some());
    assert!(step_result.completed_at.is_some());
}

#[tokio::test]
async fn test_api_executor_timeout_handling() {
    let executor = ApiExecutor::new();
    let step = PipelineStep {
        step_id: "test-timeout".to_string(),
        order: 1,
        step_type: StepType::Api,
        cmd: "http://example.com/api/timeout".to_string(),
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

    let result = executor.execute(&step).await;
    assert!(result.is_ok());

    let step_result = result.unwrap();
    assert_eq!(step_result.step_id, "test-timeout");
}

#[tokio::test]
async fn test_api_executor_empty_command() {
    let executor = ApiExecutor::new();
    let step = PipelineStep {
        step_id: "test-empty".to_string(),
        order: 1,
        step_type: StepType::Api,
        cmd: "".to_string(),  // Empty command should be handled
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

    // Empty command should result in an error
    let result = executor.execute(&step).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_api_executor_https_url() {
    let executor = ApiExecutor::new();
    let step = PipelineStep {
        step_id: "test-https".to_string(),
        order: 1,
        step_type: StepType::Api,
        cmd: "https://example.com/api/test".to_string(),
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

    let result = executor.execute(&step).await;
    assert!(result.is_ok());

    let step_result = result.unwrap();
    assert_eq!(step_result.step_id, "test-https");
    assert!(step.cmd.starts_with("https://"));
}

#[tokio::test]
async fn test_api_executor_invalid_url() {
    let executor = ApiExecutor::new();
    let step = PipelineStep {
        step_id: "test-invalid-url".to_string(),
        order: 1,
        step_type: StepType::Api,
        cmd: "not-a-url".to_string(),  // Invalid URL format
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

    let result = executor.execute(&step).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_api_executor_with_environment() {
    let executor = ApiExecutor::new();
    let mut env = HashMap::new();
    env.insert("API_KEY".to_string(), "test-key".to_string());
    env.insert("X-CUSTOM-HEADER".to_string(), "custom-value".to_string());

    let step = PipelineStep {
        step_id: "test-with-env".to_string(),
        order: 1,
        step_type: StepType::Api,
        cmd: "http://example.com/api/endpoint".to_string(),
        env: env.clone(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: SecurityPolicy::default(),
        timeout_seconds:10,
        artifact_collection: None,
    };

    let result = executor.execute(&step).await;
    assert!(result.is_ok());

    let step_result = result.unwrap();
    assert_eq!(step_result.step_id, "test-with-env");
    assert_eq!(env.len(), 2);
    assert!(env.contains_key("API_KEY"));
    assert!(env.contains_key("X-CUSTOM-HEADER"));
}

#[tokio::test]
async fn test_api_executor_with_security_policy() {
    let executor = ApiExecutor::new();
    let security_policy = SecurityPolicy {
        allow_sudo: false,
        forbidden_cmds: vec!["dd".to_string(), "rm".to_string()],
        allowed_dirs: vec!["/tmp".to_string()],
        network_policy: Default::default(),
    };

    let step = PipelineStep {
        step_id: "test-security".to_string(),
        order: 1,
        step_type: StepType::Api,
        cmd: "http://example.com/api/endpoint".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: security_policy.clone(),
        timeout_seconds: 10,
        artifact_collection: None,
    };

    let result = executor.execute(&step).await;
    assert!(result.is_ok());

    let step_result = result.unwrap();
    assert_eq!(step_result.step_id, "test-security");
    assert_eq!(security_policy.forbidden_cmds.len(), 2);
    assert_eq!(security_policy.allowed_dirs.len(), 1);
    assert!(!security_policy.allow_sudo);
}

#[tokio::test]
async fn test_api_executor_zero_timeout() {
    let executor = ApiExecutor::new();
    let step = PipelineStep {
        step_id: "test-zero-timeout".to_string(),
        order: 1,
        step_type: StepType::Api,
        cmd: "http://example.com/api/test".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: SecurityPolicy::default(),
        timeout_seconds: 0,
        artifact_collection: None,
    };

    let result = executor.execute(&step).await;
    assert!(result.is_ok());

    let step_result = result.unwrap();
    assert_eq!(step_result.step_id, "test-zero-timeout");
}
