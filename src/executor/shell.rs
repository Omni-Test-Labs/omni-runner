use anyhow::{Context, Result};
use std::process::Stdio;
use std::time::Instant;
use tokio::process::Command;
use tracing::{info, warn};

use crate::models::{PipelineStep, StepResult, TaskStatus};

/// Shell command executor
pub struct ShellExecutor;

impl ShellExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ShellExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl super::Executor for ShellExecutor {
    async fn execute(&self, step: &PipelineStep) -> Result<StepResult> {
        let start_time = Instant::now();
        let step_id = step.step_id.clone();
        let cmd_parts = shlex::split(step.cmd.as_str())
            .context("Failed to parse command")?;

        info!("Executing shell command: {}", step.cmd);

        let mut command = Command::new(&cmd_parts[0]);
        if cmd_parts.len() > 1 {
            command.args(&cmd_parts[1..]);
        }

        if let Some(ref dir) = step.working_dir {
            command.current_dir(dir);
        }

        for (key, value) in &step.env {
            command.env(key, value);
        }

        let child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn process")?;

        let timeout = tokio::time::Duration::from_secs(step.timeout_seconds);
        let output = tokio::time::timeout(timeout, child.wait_with_output()).await;

        match output {
            Ok(output_result) => {
                let output = output_result?;
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                let exit_code = output.status.code();
                let duration = start_time.elapsed().as_secs_f64();

                let status = match exit_code {
                    Some(0) => TaskStatus::Success,
                    Some(_) => TaskStatus::Failed,
                    None => TaskStatus::Crashed,
                };

                info!("Command completed with exit code: {:?}", exit_code);

                let status_for_error = status.clone();
                let mut context = std::collections::HashMap::new();
                context.insert("cmd".to_string(), step.cmd.clone());
                context.insert("stderr_preview".to_string(), stderr.chars().take(200).collect::<String>());

                Ok(StepResult {
                    step_id,
                    status,
                    started_at: Some(chrono::Utc::now().to_rfc3339()),
                    completed_at: Some(chrono::Utc::now().to_rfc3339()),
                    duration_seconds: Some(duration),
                    exit_code,
                    signal: None,
                    log_path: None,
                    log_url: None,
                    stdout_lines: Some(stdout.lines().count() as u64),
                    stderr_lines: Some(stderr.lines().count() as u64),
                    artifact_urls: Vec::new(),
                    resource_usage: None,
                    retry_count: 0,
                    error: if status_for_error != TaskStatus::Success {
                        Some(crate::models::ErrorInfo {
                            error_type: "ExecutionError".to_string(),
                            message: format!("Command failed with exit code {:?}", exit_code),
                            stack_trace: stderr.trim().to_string().into(),
                            context,
                        })
                    } else {
                        None
                    },
                    reason: None,
                })
            }
            Err(_) => {
                let duration = start_time.elapsed().as_secs_f64();

                warn!("Command timed out after {} seconds", duration);

                let mut context = std::collections::HashMap::new();
                context.insert("cmd".to_string(), step.cmd.clone());
                context.insert("timeout_seconds".to_string(), step.timeout_seconds.to_string());

                Ok(StepResult {
                    step_id,
                    status: TaskStatus::Timeout,
                    started_at: Some(chrono::Utc::now().to_rfc3339()),
                    completed_at: Some(chrono::Utc::now().to_rfc3339()),
                    duration_seconds: Some(duration),
                    exit_code: None,
                    signal: Some("SIGKILL".to_string()),
                    log_path: None,
                    log_url: None,
                    stdout_lines: None,
                    stderr_lines: None,
                    artifact_urls: Vec::new(),
                    resource_usage: None,
                    retry_count: 0,
                    error: Some(crate::models::ErrorInfo {
                        error_type: "TimeoutError".to_string(),
                        message: format!("Command timed out after {} seconds", duration),
                        stack_trace: None,
                        context,
                    }),
                    reason: Some("Task timed out".to_string()),
                })
            }
        }
    }
}
