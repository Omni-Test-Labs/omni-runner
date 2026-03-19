use anyhow::Result;
use std::collections::HashMap;

use crate::executor::{Executor, ExecutorType};
use crate::models::{PipelineStep, StepResult, TaskManifest, TaskStatus};

pub struct PipelineEngine;

impl PipelineEngine {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PipelineEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineEngine {
    pub async fn execute(&self, manifest: &TaskManifest) -> Result<Vec<StepResult>> {
        let mut results: Vec<StepResult> = Vec::new();
        let mut dependency_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut completed_steps: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut failed_must_pass: bool = false;

        for step in &manifest.pipeline {
            if !step.depends_on.is_empty() {
                dependency_map.insert(step.step_id.clone(), step.depends_on.clone());
            }
        }
        
        let ordered_steps = self.topological_sort(&manifest.pipeline)?;

        for step in ordered_steps {
            if failed_must_pass && step.depends_on.is_empty() {
                break;
            }

            if !self.can_execute_step(&step, &completed_steps, &dependency_map) {
                results.push(StepResult {
                    step_id: step.step_id.clone(),
                    status: TaskStatus::Skipped,
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
                    error: None,
                    reason: Some("Skipped due to dependency failure".to_string()),
                });
                
                continue;
            }
            
            if !self.should_execute_step(&step, &results) {
                results.push(StepResult {
                    step_id: step.step_id.clone(),
                    status: TaskStatus::Skipped,
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
                    error: None,
                    reason: Some("Skipped because upstream step failed".to_string()),
                });
                
                continue;
            }
            
            let executor = self.get_executor(&step)?;
            let result = executor.execute(&step).await?;

            completed_steps.insert(step.step_id.clone());

            let success = result.status == TaskStatus::Success;
            if !success && step.must_pass {
                failed_must_pass = true;
            }

            results.push(result);
        }
        
        Ok(results)
    }
    
    fn get_executor(&self, step: &PipelineStep) -> Result<ExecutorType> {
        match step.step_type {
            crate::models::StepType::Python => Ok(ExecutorType::Python(crate::executor::PythonExecutor::new())),
            crate::models::StepType::Binary => Ok(ExecutorType::Binary(crate::executor::BinaryExecutor::new())),
            crate::models::StepType::Shell => Ok(ExecutorType::Shell(crate::executor::ShellExecutor::new())),
            crate::models::StepType::Api => Ok(ExecutorType::Api(crate::executor::ApiExecutor::new())),
        }
    }
    
    fn should_execute_step(&self, step: &PipelineStep, previous_results: &[StepResult]) -> bool {
        if step.always_run {
            return true;
        }
        
        for dep_id in &step.depends_on {
            if let Some(dep_result) = previous_results.iter().find(|r| r.step_id == *dep_id) {
                if !matches!(dep_result.status, TaskStatus::Success) {
                    return false;
                }
            }
        }
        
        true
    }
    
    fn can_execute_step(
        &self,
        step: &PipelineStep,
        completed: &std::collections::HashSet<String>,
        dependencies: &HashMap<String, Vec<String>>,
    ) -> bool {
        if let Some(deps) = dependencies.get(&step.step_id) {
            for dep_id in deps {
                if !completed.contains(dep_id) {
                    return false;
                }
            }
        }
        true
    }
    
    fn topological_sort(&self, pipeline: &[PipelineStep]) -> Result<Vec<PipelineStep>> {
        let mut in_degree: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let mut adj_list: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

        for step in pipeline {
            in_degree.insert(step.step_id.clone(), step.depends_on.len());
            for dep_id in &step.depends_on {
                adj_list.entry(dep_id.clone())
                    .or_insert_with(Vec::new)
                    .push(step.step_id.clone());
            }
        }

        let mut result = Vec::new();
        let mut initial_steps: Vec<&PipelineStep> = pipeline
            .iter()
            .filter(|s| s.depends_on.is_empty())
            .collect();
        initial_steps.sort_by_key(|s| s.order);

        let mut queue: std::collections::VecDeque<String> = initial_steps
            .into_iter()
            .map(|s| s.step_id.clone())
            .collect();

        while let Some(current_id) = queue.pop_front() {
            if let Some(step) = pipeline.iter().find(|s| s.step_id == current_id) {
                result.push(step.clone());
            }

            if let Some(next_steps) = adj_list.remove(&current_id) {
                for next_id in next_steps {
                    if let Some(degree) = in_degree.get_mut(&next_id) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(next_id);
                        }
                    }
                }
            }
        }

        Ok(result)
    }
}
