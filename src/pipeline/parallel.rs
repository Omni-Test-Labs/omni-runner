use super::dag::DagValidator;
use crate::executor::ExecutorType;
use crate::models::{PipelineStep, StepResult};
use std::collections::HashSet;

pub struct ParallelEngine {
    max_parallelism: usize,
    dag_validator: DagValidator,
}

impl ParallelEngine {
    pub fn new(max_parallelism: usize) -> Self {
        Self {
            max_parallelism,
            dag_validator: DagValidator::new(),
        }
    }

    pub async fn execute<F>(
        &self,
        pipeline: &[PipelineStep],
        executor_factory: F,
    ) -> Result<Vec<StepResult>, anyhow::Error>
    where
        F: Fn(&PipelineStep) -> ExecutorType + Clone,
    {
        if pipeline.is_empty() {
            return Ok(Vec::new());
        }

        self.dag_validator.validate_cycle(pipeline)
            .map_err(|e| anyhow::anyhow!(e))?;

        let step_map: std::collections::HashMap<String, PipelineStep> = pipeline
            .iter()
            .map(|s| (s.step_id.clone(), s.clone()))
            .collect();

        let mut completed: HashSet<String> = HashSet::new();
        let mut results: Vec<StepResult> = Vec::new();

        loop {
            let ready_steps = self.dag_validator.get_ready_steps(pipeline, &completed);

            if ready_steps.is_empty() {
                if completed.len() == pipeline.len() {
                    break;
                }
                return Err(anyhow::anyhow!(
                    "Pipeline stuck: {} completed out of {} steps",
                    completed.len(),
                    pipeline.len()
                ));
            }

            let batch_results = self
                .execute_batch(&ready_steps, &executor_factory, &step_map)
                .await?;

            for result in &batch_results {
                completed.insert(result.step_id.clone());
            }

            results.extend(batch_results);
        }

        let step_order_map: std::collections::HashMap<String, u32> = pipeline
            .iter()
            .map(|s| (s.step_id.clone(), s.order))
            .collect();

        results.sort_by_key(|r| step_order_map.get(&r.step_id).copied().unwrap_or(u32::MAX));

        Ok(results)
    }

    async fn execute_batch<F>(
        &self,
        steps: &[PipelineStep],
        executor_factory: &F,
        step_map: &std::collections::HashMap<String, PipelineStep>,
    ) -> Result<Vec<StepResult>, anyhow::Error>
    where
        F: Fn(&PipelineStep) -> ExecutorType + Clone,
    {
        use crate::executor::Executor;
        use crate::models::TaskStatus;

        let mut results = Vec::new();
        let mut tasks = std::collections::VecDeque::new();

        for step in steps {
            let executor = executor_factory(step);
            let step_id = step.step_id.clone();
            let step_clone = step_map.get(&step_id).cloned().unwrap();

            let handle = tokio::spawn(async move {
                executor.execute(&step_clone).await
            });

            tasks.push_back(handle);

            if tasks.len() >= self.max_parallelism {
                let handle = tasks.pop_front().unwrap();
                match handle.await {
                    Ok(Ok(result)) => results.push(result),
                    Ok(Err(e)) => {
                        return Err(anyhow::anyhow!("Task execution failed: {}", e));
                    }
                    Err(e) => {
                        return Err(anyhow::anyhow!("Task join failed: {}", e));
                    }
                }
            }
        }

        while let Some(handle) = tasks.pop_front() {
            match handle.await {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => {
                    return Err(anyhow::anyhow!("Task execution failed: {}", e));
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("Task join failed: {}", e));
                }
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::StepType;

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
            failure_policy: crate::models::FailurePolicy::Stop,
            security_policy: Default::default(),
            timeout_seconds: 60,
            device_lock_timeout_seconds: 300,
            artifact_collection: None,
        }
    }

    #[tokio::test]
    async fn test_parallel_execution_independent_steps() {
        let engine = ParallelEngine::new(2);
        let pipeline = vec![
            create_test_step("step1", 1, StepType::Shell, vec![]),
            create_test_step("step2", 2, StepType::Shell, vec![]),
            create_test_step("step3", 3, StepType::Shell, vec!["step1", "step2"]),
        ];

        let executor_factory = |_step: &PipelineStep| -> ExecutorType {
            ExecutorType::Shell(crate::executor::ShellExecutor::new())
        };

        let results = engine.execute(&pipeline, executor_factory).await;
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_parallel_execution_dependency_order() {
        let engine = ParallelEngine::new(2);
        let pipeline = vec![
            create_test_step("step1", 1, StepType::Shell, vec![]),
            create_test_step("step2", 2, StepType::Shell, vec!["step1"]),
            create_test_step("step3", 3, StepType::Shell, vec!["step1"]),
        ];

        let executor_factory = |_step: &PipelineStep| -> ExecutorType {
            ExecutorType::Shell(crate::executor::ShellExecutor::new())
        };

        let results = engine.execute(&pipeline, executor_factory).await;
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_parallel_execution_cycle_detection() {
        let engine = ParallelEngine::new(2);
        let pipeline = vec![
            create_test_step("step1", 1, StepType::Shell, vec!["step2"]),
            create_test_step("step2", 2, StepType::Shell, vec!["step1"]),
        ];

        let executor_factory = |_step: &PipelineStep| -> ExecutorType {
            ExecutorType::Shell(crate::executor::ShellExecutor::new())
        };

        let results = engine.execute(&pipeline, executor_factory).await;
        assert!(results.is_err());
        assert!(results.unwrap_err().to_string().contains("Cycle"));
    }

    #[tokio::test]
    async fn test_empty_pipeline() {
        let engine = ParallelEngine::new(2);
        let pipeline: Vec<PipelineStep> = vec![];

        let executor_factory = |_step: &PipelineStep| -> ExecutorType {
            ExecutorType::Shell(crate::executor::ShellExecutor::new())
        };

        let results = engine.execute(&pipeline, executor_factory).await;
        assert!(results.is_ok());
        assert!(results.unwrap().is_empty());
    }
}
