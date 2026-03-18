use anyhow::Result;
use std::path::Path;
use tokio::process::Command;

use crate::models::{PipelineStep, StepResult};

/// Python script executor
pub struct PythonExecutor;

impl PythonExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PythonExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl super::Executor for PythonExecutor {
    async fn execute(&self, step: &PipelineStep) -> Result<StepResult> {
        let python_cmd = "python";
        
        let mut command = Command::new(python_cmd);
        command.arg(&step.cmd);
        
        if let Some(ref dir) = step.working_dir {
            command.current_dir(dir);
        }
        
        for (key, value) in &step.env {
            command.env(key, value);
        }
        
        let output = command.output().await?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let exit_code = output.status.code();
        
        Ok(StepResult {
            step_id: step.step_id.clone(),
            status: if exit_code == Some(0) {
                crate::models::TaskStatus::Success
            } else {
                crate::models::TaskStatus::Failed
            },
            started_at: Some(chrono::Utc::now().to_rfc3339()),
            completed_at: Some(chrono::Utc::now().to_rfc3339()),
            duration_seconds: None,
            exit_code,
            signal: None,
            log_path: None,
            log_url: None,
            stdout_lines: Some(stdout.lines().count() as u64),
            stderr_lines: None,
            artifact_urls: Vec::new(),
            resource_usage: None,
            retry_count: 0,
            error: None,
            reason: None,
        })
    }
}
