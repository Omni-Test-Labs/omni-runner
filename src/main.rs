mod api;
mod config;
mod models;
mod executor;
mod pipeline;
mod security;
mod utils;

use anyhow::Result;
use chrono::Utc;
use tokio::signal;
use tracing::{info, warn, error};

use api::client::ApiClient;
use config::RunnerConfig;
use models::{DeviceInfo, ExecutionResult, Heartbeat, RunnerStatus, Summary};
use pipeline::engine::PipelineEngine;
use utils::logging::init_logging;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    info!("omni-runner v0.1.0 - Starting...");

    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config".to_string());
    let config = config::load_config(&config_path)?;

    info!("Loaded configuration for device: {}", config.device.device_id);
    info!("Server URL: {}", config.server.base_url);

    let api_client = ApiClient::new(
        config.server.base_url.clone(),
        config.device.device_id.clone(),
        config.server.api_key.clone(),
    )?;

    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
        },
        _ = run_main_loop(&api_client, &config) => {
            info!("Main loop exited");
        },
    };

    Ok(())
}

async fn run_main_loop(api_client: &ApiClient, config: &RunnerConfig) -> Result<()> {
    let device_id = &config.device.device_id;
    let heartbeat_interval = tokio::time::Duration::from_secs(config.polling.heartbeat_interval_seconds);
    let poll_interval = tokio::time::Duration::from_secs(config.polling.interval_seconds);

    info!("Starting main loop - poll interval: {}s, heartbeat: {}s",
          config.polling.interval_seconds,
          config.polling.heartbeat_interval_seconds);

    let mut heartbeat_timer = tokio::time::interval(heartbeat_interval);
    let mut poll_timer = tokio::time::interval(poll_interval);

    loop {
        tokio::select! {
            _ = heartbeat_timer.tick() => {
                if let Err(e) = send_heartbeat(api_client, device_id).await {
                    warn!("Failed to send heartbeat: {}", e);
                }
            }

            _ = poll_timer.tick() => {
                if let Err(e) = poll_and_execute_task(api_client, config).await {
                    error!("Task execution failed: {}", e);
                }
            }
        }
    }
}

async fn send_heartbeat(api_client: &ApiClient, device_id: &str) -> Result<()> {
    let heartbeat = Heartbeat {
        device_id: device_id.to_string(),
        runner_version: "0.1.0".to_string(),
        status: RunnerStatus::Idle,
        current_task_id: None,
        current_task_progress: 0.0,
        system_resources: models::SystemResources {
            cpu_percent: get_cpu_usage(),
            memory_used_mb: get_memory_used_mb(),
            memory_total_mb: get_memory_total_mb(),
            disk_used_gb: get_disk_used_gb(),
            disk_total_gb: get_disk_total_gb(),
        },
        capabilities: models::Capabilities {
            supported_step_types: vec![
                models::StepType::Python,
                models::StepType::Binary,
                models::StepType::Shell,
                models::StepType::Api,
            ],
            has_oob_capture: false,
            has_gpu: false,
            gpu_model: None,
            oob_methods: vec![],
        },
        last_report: Utc::now().to_rfc3339(),
    };

    api_client.send_heartbeat(&heartbeat).await
}

async fn poll_and_execute_task(api_client: &ApiClient, config: &RunnerConfig) -> Result<()> {
    match api_client.poll_for_task().await? {
        None => {
            tracing::debug!("No pending tasks");
        }
        Some(mut manifest) => {
            info!("Found task: {} (priority: {:?})", manifest.task_id, manifest.priority);

            api_client.assign_task(&manifest.task_id).await?;

            let start_time = Utc::now();
            let pipeline_engine = PipelineEngine::new();
            let step_results = pipeline_engine.execute(&manifest).await?;
            let end_time = Utc::now();

            let successful_steps = step_results.iter()
                .filter(|r| matches!(r.status, models::TaskStatus::Success))
                .count() as u32;

            let failed_steps = step_results.iter()
                .filter(|r| matches!(r.status, models::TaskStatus::Failed | models::TaskStatus::Crashed | models::TaskStatus::Timeout))
                .count() as u32;

            let skipped_steps = step_results.iter()
                .filter(|r| matches!(r.status, models::TaskStatus::Skipped))
                .count() as u32;

            let overall_status = if failed_steps == 0 {
                models::TaskStatus::Success
            } else {
                models::TaskStatus::Failed
            };

            let result = ExecutionResult {
                schema_version: "1.0.0".to_string(),
                task_id: manifest.task_id.clone(),
                status: overall_status.clone(),
                started_at: start_time.to_rfc3339(),
                completed_at: Some(end_time.to_rfc3339()),
                duration_seconds: (end_time - start_time).num_seconds() as f64,
                device_info: DeviceInfo {
                    device_id: config.device.device_id.clone(),
                    hostname: config.device.hostname.clone(),
                    ip_address: None,
                    os_version: None,
                    runner_version: "0.1.0".to_string(),
                },
                steps: step_results.clone(),
                summary: Summary {
                    total_steps: manifest.pipeline.len() as u32,
                    successful_steps,
                    failed_steps,
                    skipped_steps,
                    crashed_steps: 0,
                    total_duration_seconds: (end_time - start_time).num_seconds() as f64,
                    total_artifacts: 0,
                    total_log_lines: 0,
                },
                ai_rca: None,
                forensics: models::Forensics::default(),
            };

            api_client.report_result(&result).await?;
            info!("Task {} completed with status: {:?}", manifest.task_id, overall_status);
        }
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn get_cpu_usage() -> f64 {
    use std::fs;
    if let Ok(proc_stat) = fs::read_to_string("/proc/stat") {
        let lines: Vec<&str> = proc_stat.lines().collect();
        if let Some(line) = lines.first() {
            let parts: Vec<u64> = line.split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();

            if parts.len() >= 4 {
                let idle = parts[3];
                let total: u64 = parts.iter().sum();
                if total > 0 {
                    return ((total - idle) as f64 / total as f64) * 100.0;
                }
            }
        }
    }
    0.0
}

#[cfg(target_os = "linux")]
fn get_memory_used_mb() -> u64 {
    use std::fs;

    let available_kb = if let Ok(proc_meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in proc_meminfo.lines() {
            if line.starts_with("MemAvailable:") {
                if let Some(avail_str) = line.split(':').nth(1) {
                    if let Some(val_str) = avail_str.trim().split_whitespace().next() {
                        if let Ok(mem_kb) = val_str.parse::<u64>() {
                            return mem_kb;
                        }
                    }
                }
            }
        }
        0
    } else {
        0
    };

    let total_kb = if let Ok(proc_meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in proc_meminfo.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(total_str) = line.split(':').nth(1) {
                    if let Some(val_str) = total_str.trim().split_whitespace().next() {
                        if let Ok(mem_kb) = val_str.parse::<u64>() {
                            return mem_kb;
                        }
                    }
                }
            }
        }
        8192 * 1024
    } else {
        8192 * 1024
    };

    if total_kb > available_kb {
        (total_kb - available_kb) / 1024
    } else {
        0
    }
}

#[cfg(not(target_os = "linux"))]
fn get_cpu_usage() -> f64 { 0.0 }

#[cfg(not(target_os = "linux"))]
fn get_memory_used_mb() -> u64 { 0 }

fn get_memory_total_mb() -> u64 {
    8192
}

fn get_disk_used_gb() -> u64 {
    50
}

fn get_disk_total_gb() -> u64 {
    500
}