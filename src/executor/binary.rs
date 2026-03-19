use anyhow::Result;
use std::process::Stdio;
use tokio::process::Command;

use crate::models::{PipelineStep, StepResult};

#[derive(Debug)]
pub struct BinaryExecutor;

impl BinaryExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BinaryExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl super::Executor for BinaryExecutor {
    async fn execute(&self, step: &PipelineStep) -> Result<StepResult> {
        let mut command = Command::new(&step.cmd);
        
        if let Some(ref dir) = step.working_dir {
            command.current_dir(dir);
        }
        
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        
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
