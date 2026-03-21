use crate::models::{FailurePolicy, PipelineStep, StepResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    ContinueExecution,
    StopExecution,
    SkipDependents(Vec<String>),
}

pub struct FailurePolicyExecutor;

impl FailurePolicyExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FailurePolicyExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl FailurePolicyExecutor {
    pub fn should_continue(
        &self,
        policy: &FailurePolicy,
        failed_step: &StepResult,
        pipeline: &[PipelineStep],
    ) -> Decision {
        match policy {
            FailurePolicy::Continue => Decision::ContinueExecution,
            FailurePolicy::Stop => Decision::StopExecution,
            FailurePolicy::Skip => {
                let dependents = self.get_dependent_steps(&failed_step.step_id, pipeline);
                Decision::SkipDependents(dependents)
            }
            FailurePolicy::DiagnoseOnly => Decision::StopExecution,
        }
    }

    pub fn get_dependent_steps(
        &self,
        failed_step_id: &str,
        pipeline: &[PipelineStep],
    ) -> Vec<String> {
        let mut dependents: Vec<String> = Vec::new();
        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(failed_step_id.to_string());
        visited.insert(failed_step_id.to_string());

        while let Some(current) = queue.pop_front() {
            for step in pipeline {
                if step.depends_on.contains(&current) && !visited.contains(&step.step_id) {
                    visited.insert(step.step_id.clone());
                    dependents.push(step.step_id.clone());
                    queue.push_back(step.step_id.clone());
                }
            }
        }

        dependents
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::StepType;
    use crate::models::TaskStatus;

    fn create_test_step(
        step_id: &str,
        order: u32,
        step_type: StepType,
        depends_on: Vec<&str>,
    ) -> PipelineStep {
        PipelineStep {
            step_id: step_id.to_string(),
            order,
            step_type,
            cmd: "echo test".to_string(),
            env: std::collections::HashMap::new(),
            working_dir: None,
            must_pass: true,
            depends_on: depends_on.into_iter().map(|s| s.to_string()).collect(),
            always_run: false,
            retry_policy: None,
            failure_policy: FailurePolicy::Stop,
            security_policy: Default::default(),
            timeout_seconds: 60,
            device_lock_timeout_seconds: 300,
            artifact_collection: None,
        }
    }

    fn create_failed_result(step_id: &str) -> StepResult {
        StepResult {
            step_id: step_id.to_string(),
            status: TaskStatus::Failed,
            started_at: None,
            completed_at: None,
            duration_seconds: None,
            exit_code: None,
            signal: None,
            log_path: None,
            log_url: None,
            stdout_lines: None,
            stderr_lines: None,
            artifact_urls: Vec::new(),
            resource_usage: None,
            retry_count: 0,
            error: Some(crate::models::ErrorInfo {
                error_type: "TestError".to_string(),
                message: "Test error".to_string(),
                stack_trace: None,
                context: std::collections::HashMap::new(),
            }),
            reason: Some("Test failure".to_string()),
        }
    }

    #[test]
    fn test_should_continue_continue_policy() {
        let executor = FailurePolicyExecutor::new();
        let pipeline = vec![];
        let failed = create_failed_result("step1");

        let decision = executor.should_continue(&FailurePolicy::Continue, &failed, &pipeline);
        assert!(matches!(decision, Decision::ContinueExecution));
    }

    #[test]
    fn test_should_continue_stop_policy() {
        let executor = FailurePolicyExecutor::new();
        let pipeline = vec![];
        let failed = create_failed_result("step1");

        let decision = executor.should_continue(&FailurePolicy::Stop, &failed, &pipeline);
        assert!(matches!(decision, Decision::StopExecution));
    }

    #[test]
    fn test_should_continue_skip_policy() {
        let executor = FailurePolicyExecutor::new();
        let pipeline = vec![
            create_test_step("step1", 1, StepType::Shell, vec![]),
            create_test_step("step2", 2, StepType::Shell, vec!["step1"]),
            create_test_step("step3", 3, StepType::Shell, vec!["step2"]),
        ];
        let failed = create_failed_result("step1");

        let decision = executor.should_continue(&FailurePolicy::Skip, &failed, &pipeline);
        match decision {
            Decision::SkipDependents(step_ids) => {
                assert_eq!(step_ids.len(), 2);
                assert!(step_ids.contains(&"step2".to_string()));
                assert!(step_ids.contains(&"step3".to_string()));
            }
            _ => panic!("Expected SkipDependents"),
        }
    }

    #[test]
    fn test_get_dependent_steps() {
        let executor = FailurePolicyExecutor::new();
        let pipeline = vec![
            create_test_step("step1", 1, StepType::Shell, vec![]),
            create_test_step("step2", 2, StepType::Shell, vec!["step1"]),
            create_test_step("step3", 3, StepType::Shell, vec!["step2"]),
            create_test_step("step4", 4, StepType::Shell, vec!["step1"]),
        ];

        let dependents = executor.get_dependent_steps("step1", &pipeline);
        assert_eq!(dependents.len(), 3);
        assert!(dependents.contains(&"step2".to_string()));
        assert!(dependents.contains(&"step3".to_string()));
        assert!(dependents.contains(&"step4".to_string()));
    }

    #[test]
    fn test_get_dependent_steps_no_dependents() {
        let executor = FailurePolicyExecutor::new();
        let pipeline = vec![
            create_test_step("step1", 1, StepType::Shell, vec![]),
            create_test_step("step2", 2, StepType::Shell, vec![]),
        ];

        let dependents = executor.get_dependent_steps("step1", &pipeline);
        assert!(dependents.is_empty());
    }
}
