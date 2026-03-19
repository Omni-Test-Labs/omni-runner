use std::collections::HashMap;

use omni_runner::security::PolicyValidator;
use omni_runner::models::*;

#[test]
fn test_security_policy_validator_allow_command() {
    let policy = SecurityPolicy {
        allow_sudo: false,
        forbidden_cmds: vec!["rm".to_string(), "dd".to_string()],
        allowed_dirs: vec![],
        network_policy: NetworkPolicy {
            allow_internet: false,
            allowed_hosts: vec![],
        },
    };

    let step = PipelineStep {
        step_id: "test".to_string(),
        order: 1,
        step_type: StepType::Shell,
        cmd: "echo hello".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: policy.clone(),
        timeout_seconds: 10,
        artifact_collection: None,
    };

    let validator = PolicyValidator::new(&policy);
    assert!(validator.validate(&step).is_ok());
}

#[test]
fn test_security_policy_validator_forbidden_command() {
    let policy = SecurityPolicy {
        allow_sudo: false,
        forbidden_cmds: vec!["rm".to_string(), "dd".to_string()],
        allowed_dirs: vec![],
        network_policy: NetworkPolicy {
            allow_internet: false,
            allowed_hosts: vec![],
        },
    };

    let step = PipelineStep {
        step_id: "test".to_string(),
        order: 1,
        step_type: StepType::Shell,
        cmd: "rm /tmp/test".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: policy.clone(),
        timeout_seconds: 10,
        artifact_collection: None,
    };

    let validator = PolicyValidator::new(&policy);
    assert!(validator.validate(&step).is_err());
}

#[test]
fn test_security_policy_validator_sudo_disabled() {
    let policy = SecurityPolicy {
        allow_sudo: false,
        forbidden_cmds: vec![],
        allowed_dirs: vec![],
        network_policy: NetworkPolicy {
            allow_internet: false,
            allowed_hosts: vec![],
        },
    };

    let step = PipelineStep {
        step_id: "test".to_string(),
        order: 1,
        step_type: StepType::Shell,
        cmd: "sudo rm /tmp/test".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: policy.clone(),
        timeout_seconds: 10,
        artifact_collection: None,
    };

    let validator = PolicyValidator::new(&policy);
    assert!(validator.validate(&step).is_err());
}

#[test]
fn test_security_policy_validator_sudo_enabled() {
    let policy = SecurityPolicy {
        allow_sudo: true,
        forbidden_cmds: vec![],
        allowed_dirs: vec![],
        network_policy: NetworkPolicy {
            allow_internet: false,
            allowed_hosts: vec![],
        },
    };

    let step = PipelineStep {
        step_id: "test".to_string(),
        order: 1,
        step_type: StepType::Shell,
        cmd: "sudo ls".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: policy.clone(),
        timeout_seconds: 10,
        artifact_collection: None,
    };

    let validator = PolicyValidator::new(&policy);
    assert!(validator.validate(&step).is_ok());
}

#[test]
fn test_security_policy_validator_allowed_dir() {
    let policy = SecurityPolicy {
        allow_sudo: false,
        forbidden_cmds: vec![],
        allowed_dirs: vec!["/tmp".to_string(), "/home/user".to_string()],
        network_policy: NetworkPolicy {
            allow_internet: false,
            allowed_hosts: vec![],
        },
    };

    let step = PipelineStep {
        step_id: "test".to_string(),
        order: 1,
        step_type: StepType::Shell,
        cmd: "ls".to_string(),
        env: HashMap::new(),
        working_dir: Some("/tmp/workspace".to_string()),
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: policy.clone(),
        timeout_seconds: 10,
        artifact_collection: None,
    };

    let validator = PolicyValidator::new(&policy);
    assert!(validator.validate(&step).is_ok());
}

#[test]
fn test_security_policy_validator_disallowed_dir() {
    let policy = SecurityPolicy {
        allow_sudo: false,
        forbidden_cmds: vec![],
        allowed_dirs: vec!["/tmp".to_string()],
        network_policy: NetworkPolicy {
            allow_internet: false,
            allowed_hosts: vec![],
        },
    };

    let step = PipelineStep {
        step_id: "test".to_string(),
        order: 1,
        step_type: StepType::Shell,
        cmd: "ls".to_string(),
        env: HashMap::new(),
        working_dir: Some("/etc".to_string()),
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: policy.clone(),
        timeout_seconds: 10,
        artifact_collection: None,
    };

    let validator = PolicyValidator::new(&policy);
    assert!(validator.validate(&step).is_err());
}

#[test]
fn test_security_policy_validator_no_working_dir() {
    let policy = SecurityPolicy {
        allow_sudo: false,
        forbidden_cmds: vec![],
        allowed_dirs: vec!["/tmp".to_string()],
        network_policy: NetworkPolicy {
            allow_internet: false,
            allowed_hosts: vec![],
        },
    };

    let step = PipelineStep {
        step_id: "test".to_string(),
        order: 1,
        step_type: StepType::Shell,
        cmd: "ls".to_string(),
        env: HashMap::new(),
        working_dir: None,
        must_pass: true,
        depends_on: vec![],
        always_run: false,
        retry_policy: None,
        security_policy: policy.clone(),
        timeout_seconds: 10,
        artifact_collection: None,
    };

    let validator = PolicyValidator::new(&policy);
    assert!(validator.validate(&step).is_ok());
}

#[test]
fn test_policy_validator_default() {
    let validator = PolicyValidator::default();
    assert!(validator.forbidden_commands.is_empty());
}
