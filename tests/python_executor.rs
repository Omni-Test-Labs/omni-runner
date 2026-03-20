use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use omni_runner::models::*;
use omni_runner::executor::*;
use omni_runner::executor::Executor as _;

fn create_temp_python_script(content: &str) -> PathBuf {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut file_path = std::env::temp_dir();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    file_path.push(format!("test_python_{}.py", timestamp));
    
    let mut file = File::create(&file_path).expect("Failed to create temp file");
    file.write_all(content.as_bytes()).expect("Failed to write to temp file");
    file.flush().expect("Failed to flush temp file");
    
    file_path
}

fn cleanup_temp_script(path: &PathBuf) {
    let _ = std::fs::remove_file(path);
}

#[test]
fn test_python_executor_default() {
    let executor = PythonExecutor::new();
    assert_eq!(format!("{:?}", executor), "PythonExecutor");
}

#[tokio::test]
async fn test_execute_simple_python_code() {
    let script_content = "print('hello from python')";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-simple".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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

    assert_eq!(result.step_id, "test-simple");
    assert_eq!(result.status, TaskStatus::Success);
    assert_eq!(result.exit_code, Some(0));
    assert!(result.stdout_lines.is_some());
    assert!(result.started_at.is_some());
    assert!(result.completed_at.is_some());
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_with_print_output() {
    let script_content = "print('line 1')\nprint('line 2')\nprint('line 3')";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-output".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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

    assert_eq!(result.status, TaskStatus::Success);
    assert_eq!(result.exit_code, Some(0));
    assert_eq!(result.stdout_lines, Some(3));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_multiline_code() {
    let script_content = "name = 'test'\nprint(f'Hello {name}!')\nprint('Done')";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-multiline".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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

    assert_eq!(result.status, TaskStatus::Success);
    assert_eq!(result.exit_code, Some(0));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_math_operations() {
    let script_content = "import math\nprint(math.sqrt(16))\nprint(2 + 2)\nprint(10 * 5)";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-math".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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

    assert_eq!(result.status, TaskStatus::Success);
    assert_eq!(result.exit_code, Some(0));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_with_syntax_error() {
    let script_content = "print('hello'";  // Missing closing quote
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-syntax-error".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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
    assert!(result.exit_code.is_some_and(|code| code != 0));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_with_runtime_error() {
    let script_content = "raise ValueError('test error')";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-runtime-error".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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
    assert!(result.exit_code.is_some_and(|code| code != 0));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_with_env_vars() {
    let mut env = HashMap::new();
    env.insert("TEST_VAR".to_string(), "test_value".to_string());
    
    let script_content = "import os\nprint(os.environ.get('TEST_VAR', 'not found'))";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-env".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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
    assert_eq!(result.exit_code, Some(0));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_env_var_usage() {
    let mut env = HashMap::new();
    env.insert("VAR1".to_string(), "value1".to_string());
    env.insert("VAR2".to_string(), "value2".to_string());
    
    let script_content = "import os\nprint(os.environ['VAR1'])\nprint(os.environ['VAR2'])";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-env-usage".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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
    assert_eq!(result.exit_code, Some(0));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_with_working_dir() {
    let script_content = "import os\nprint('Current dir:', os.getcwd())";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-working-dir".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
        env: HashMap::new(),
        working_dir: Some("/tmp".to_string()),
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
    assert_eq!(result.exit_code, Some(0));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_with_exit_code() {
    let script_content = "import sys\nsys.exit(42)";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-exit-code".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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

    assert_eq!(result.status, TaskStatus::Failed);
    assert_eq!(result.exit_code, Some(42));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_zero_exit_code() {
    let script_content = "import sys\nsys.exit(0)";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-zero-exit".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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

    assert_eq!(result.status, TaskStatus::Success);
    assert_eq!(result.exit_code, Some(0));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_nonzero_exit_code() {
    let script_content = "import sys\nsys.exit(1)";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-nonzero-exit".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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

    assert_eq!(result.status, TaskStatus::Failed);
    assert_eq!(result.exit_code, Some(1));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_with_imports() {
    let script_content = "import json\nimport sys\nprint('Python version:', sys.version_info.major)\nprint('JSON imported:', json.__name__)";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-imports".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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

    assert_eq!(result.status, TaskStatus::Success);
    assert_eq!(result.exit_code, Some(0));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_execute_string_operations() {
    let script_content = "s = 'hello world'\nprint(s.upper())\nprint(s.split())\nprint(len(s))";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-strings".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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

    assert_eq!(result.status, TaskStatus::Success);
    assert_eq!(result.exit_code, Some(0));
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_result_fields_populated() {
    let script_content = "print('test')";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-fields".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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

    assert_eq!(result.step_id, "test-fields");
    assert!(matches!(result.status, TaskStatus::Success | TaskStatus::Failed));
    assert!(result.started_at.is_some());
    assert!(result.completed_at.is_some());
    assert!(result.exit_code.is_some());
    assert!(result.stdout_lines.is_some());
    assert_eq!(result.signal, None);
    assert_eq!(result.log_path, None);
    assert_eq!(result.log_url, None);
    assert_eq!(result.artifact_urls, Vec::<String>::new());
    assert_eq!(result.retry_count, 0);
    assert!(result.error.is_none());
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_result_timestamps() {
    let script_content = "print('test')";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-timestamps".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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

    let started_at = result.started_at.expect("started_at should be set");
    let completed_at = result.completed_at.expect("completed_at should be set");

    // Verify timestamps are in RFC3339 format
    assert!(chrono::DateTime::parse_from_rfc3339(&started_at).is_ok());
    assert!(chrono::DateTime::parse_from_rfc3339(&completed_at).is_ok());
    
    cleanup_temp_script(&script_path);
}

#[tokio::test]
async fn test_stdout_lines_count() {
    let script_content = "for i in range(5):\n    print(f'Line {i}')";
    let script_path = create_temp_python_script(script_content);
    
    let executor = PythonExecutor::new();
    let step = PipelineStep {
        step_id: "test-lines".to_string(),
        order: 1,
        step_type: StepType::Python,
        cmd: script_path.to_string_lossy().to_string(),
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

    assert_eq!(result.status, TaskStatus::Success);
    assert_eq!(result.stdout_lines, Some(5));
    
    cleanup_temp_script(&script_path);
}
