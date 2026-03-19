// Task module - handles polling and executing tasks
// Testable logic separated from main loop

use anyhow::Result;
use chrono::Utc;
use tracing::info;

use crate::api::client::ApiClient;
use crate::config::RunnerConfig;
use crate::models::{DeviceInfo, ExecutionResult, Summary, TaskManifest, TaskStatus, StepResult};
use crate::pipeline::engine::PipelineEngine;

/// Polls for tasks from the server and executes them
pub async fn poll_and_execute_task(api_client: &ApiClient, config: &RunnerConfig) -> Result<()> {
    match api_client.poll_for_task().await? {
        None => {
            tracing::debug!("No pending tasks");
        }
        Some(manifest) => {
            execute_task_assignment(api_client, config, manifest).await?;
        }
    }

    Ok(())
}

/// Assigns, executes, and reports results for a single task
pub async fn execute_task_assignment(
    api_client: &ApiClient,
    config: &RunnerConfig,
    manifest: TaskManifest,
) -> Result<()> {
    info!("Found task: {} (priority: {:?})", manifest.task_id, manifest.priority);

    // Assign the task
    api_client.assign_task(&manifest.task_id).await?;

    // Execute the pipeline
    let (step_results, start_time, end_time) = execute_pipeline(&manifest).await?;

    // Calculate overall status
    let overall_status = calculate_status(&step_results);

    // Create execution result
    let result = create_execution_result(
        manifest,
        overall_status.clone(),
        start_time,
        end_time,
        config,
        step_results,
    );

    // Report result to server
    api_client.report_result(&result).await?;
    info!("Task {} completed with status: {:?}", result.task_id, overall_status);

    Ok(())
}

/// Executes the task pipeline and returns results
async fn execute_pipeline(manifest: &TaskManifest) -> Result<(Vec<StepResult>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)> {
    let start_time = Utc::now();
    let pipeline_engine = PipelineEngine::new();
    let step_results = pipeline_engine.execute(manifest).await?;
    let end_time = Utc::now();

    Ok((step_results, start_time, end_time))
}

/// Calculates overall task status from step results
fn calculate_status(step_results: &[StepResult]) -> TaskStatus {
    let failed_steps = step_results.iter()
        .filter(|s| matches!(s.status, TaskStatus::Failed | TaskStatus::Crashed | TaskStatus::Timeout))
        .count();

    if failed_steps == 0 {
        TaskStatus::Success
    } else {
        TaskStatus::Failed
    }
}

/// Creates an execution result from task execution data
fn create_execution_result(
    manifest: TaskManifest,
    overall_status: TaskStatus,
    start_time: chrono::DateTime<chrono::Utc>,
    end_time: chrono::DateTime<chrono::Utc>,
    config: &RunnerConfig,
    step_results: Vec<StepResult>,
) -> ExecutionResult {
    let duration_seconds = (end_time - start_time).num_seconds() as f64;

    let successful_steps = step_results.iter()
        .filter(|s| matches!(s.status, TaskStatus::Success))
        .count() as u32;

    let failed_steps = step_results.iter()
        .filter(|s| matches!(s.status, TaskStatus::Failed | TaskStatus::Crashed | TaskStatus::Timeout))
        .count() as u32;

    let skipped_steps = step_results.iter()
        .filter(|s| matches!(s.status, TaskStatus::Skipped))
        .count() as u32;

    ExecutionResult {
        schema_version: "1.0.0".to_string(),
        task_id: manifest.task_id.clone(),
        status: overall_status.clone(),
        started_at: start_time.to_rfc3339(),
        completed_at: Some(end_time.to_rfc3339()),
        duration_seconds,
        device_info: DeviceInfo {
            device_id: config.device.device_id.clone(),
            hostname: config.device.hostname.clone(),
            ip_address: None,
            os_version: None,
            runner_version: "0.1.0".to_string(),
        },
        steps: step_results,
        summary: Summary {
            total_steps: manifest.pipeline.len() as u32,
            successful_steps,
            failed_steps,
            skipped_steps,
            crashed_steps: 0,
            total_duration_seconds: duration_seconds,
            total_artifacts: 0,
            total_log_lines: 0,
        },
        ai_rca: None,
        forensics: crate::models::Forensics::default(),
    }
}
