use anyhow::Result;
use reqwest::Client as HttpClient;

use crate::models::{PipelineStep, StepResult};

pub struct ApiExecutor;

impl ApiExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ApiExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl super::Executor for ApiExecutor {
    async fn execute(&self, step: &PipelineStep) -> Result<StepResult> {
        let client = HttpClient::new();
        
        let output = client.post(&step.cmd)
            .send()
            .await?
            .text()
            .await?;
        
        Ok(StepResult {
            step_id: step.step_id.clone(),
            status: crate::models::TaskStatus::Success,
            started_at: Some(chrono::Utc::now().to_rfc3339()),
            completed_at: Some(chrono::Utc::now().to_rfc3339()),
            duration_seconds: None,
            exit_code: Some(0),
            signal: None,
            log_path: None,
            log_url: None,
            stdout_lines: Some(output.lines().count() as u64),
            stderr_lines: None,
            artifact_urls: Vec::new(),
            resource_usage: None,
            retry_count: 0,
            error: None,
            reason: None,
        })
    }
}
