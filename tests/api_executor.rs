use std::collections::HashMap;

use omni_runner::executor::*;
use omni_runner::models::*;

#[test]
fn test_api_executor_default() {
    let executor = ApiExecutor::new();
    assert_eq!(format!("{:?}", executor), "ApiExecutor");
}

#[tokio::test]
#[ignore]
async fn test_api_executor_simple_call() {
    // Create a step that calls a real HTTP echo service
    let executor = ApiExecutor::new();
    let step = PipelineStep {
        step_id: "test-api".to_string(),
        order: 1,
        step_type: StepType::Api,
        cmd: "https://httpbin.org/post".to_string(),
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

    assert_eq!(result.step_id, "test-api");
    assert_eq!(result.status, TaskStatus::Success);
    assert_eq!(result.exit_code, Some(0));
    assert!(result.stdout_lines.is_some());
}
