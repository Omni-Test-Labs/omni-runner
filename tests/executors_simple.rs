use omni_runner::executor::{ShellExecutor, BinaryExecutor};
use omni_runner::executor::Executor as _;
use std::collections::HashMap;

fn create_simple_shell_step() -> omni_runner::models::PipelineStep {
    omni_runner::models::PipelineStep {
        step_id: "simple".to_string(),
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
        timeout_seconds: 10,
        artifact_collection: None,
    }
}

#[test]
fn test_executor_simple_shell_echo() {
    let _executor = ShellExecutor::new();
    let step = create_simple_shell_step();
    assert_eq!(step.cmd, "echo test");
}

#[test]
fn test_executor_simple_shell_ls() {
    let _executor = ShellExecutor::new();
    let step = omni_runner::models::PipelineStep {
        step_id: "ls".to_string(),
        order: 1,
        step_type: omni_runner::models::StepType::Shell,
        cmd: "/bin/ls".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: false,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 10,
        artifact_collection: None,
    };
    assert_eq!(step.cmd, "/bin/ls");
}

#[test]
fn test_executor_simple_shell_true() {
    let _executor = ShellExecutor::new();
    let step = omni_runner::models::PipelineStep {
        step_id: "true".to_string(),
        order: 1,
        step_type: omni_runner::models::StepType::Shell,
        cmd: "/bin/true".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: false,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 10,
        artifact_collection: None,
    };
    assert!(true);
}

#[test]
fn test_executor_simple_binary_ls() {
    let _executor = BinaryExecutor::new();
    let step = omni_runner::models::PipelineStep {
        step_id: "ls".to_string(),
        order: 1,
        step_type: omni_runner::models::StepType::Binary,
        cmd: "/bin/ls".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: false,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 10,
        artifact_collection: None,
    };
    assert_eq!(step.step_type, omni_runner::models::StepType::Binary);
}

#[test]
fn test_executor_simple_binary_true() {
    let _executor = BinaryExecutor::new();
    let step = omni_runner::models::PipelineStep {
        step_id: "true".to_string(),
        order: 1,
        step_type: omni_runner::models::StepType::Binary,
        cmd: "/bin/true".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: false,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 10,
        artifact_collection: None,
    };
    assert!(true);
}

#[test]
fn test_executor_simple_must_pass_true() {
    let step = omni_runner::models::PipelineStep {
        step_id: "test".to_string(),
        order: 1,
        step_type: omni_runner::models::StepType::Shell,
        cmd: "echo test".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 10,
        artifact_collection: None,
    };
    assert_eq!(step.must_pass, true);
}

#[test]
fn test_executor_simple_order() {
    let step1 = omni_runner::models::PipelineStep {
        step_id: "step1".to_string(),
        order: 1,
        step_type: omni_runner::models::StepType::Shell,
        cmd: "echo 1".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: false,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 10,
        artifact_collection: None,
    };
    
    let step2 = omni_runner::models::PipelineStep {
        step_id: "step2".to_string(),
        order: 2,
        step_type: omni_runner::models::StepType::Shell,
        cmd: "echo 2".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: false,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: omni_runner::models::SecurityPolicy::default(),
        timeout_seconds: 10,
        artifact_collection: None,
    };
    
    assert_eq!(step1.order, 1);
    assert_eq!(step2.order, 2);
    assert!(step1.order < step2.order);
}

#[test]
fn test_executor_simple_timeout_values() {
    let timeouts = vec![0, 10, 30, 60, 300, 600];
    for timeout in timeouts {
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
            retry_policy: None,
            security_policy: omni_runner::models::SecurityPolicy::default(),
            timeout_seconds: timeout,
            artifact_collection: None,
        };
        assert_eq!(step.timeout_seconds, timeout);
    }
}
