// Heartbeat module - handles sending periodic heartbeat to server
// Testable logic separated from main loop

use anyhow::Result;
use chrono::Utc;
use tracing::info;

use crate::api::client::ApiClient;
use crate::models::{Heartbeat, RunnerStatus};

// Import system monitoring functions
use crate::system;

/// Creates and sends a heartbeat to the server
pub async fn send_heartbeat(api_client: &ApiClient, device_id: &str) -> Result<()> {
    let heartbeat = create_heartbeat(device_id)?;
    api_client.send_heartbeat(&heartbeat).await
}

/// Creates a heartbeat message with current system state
pub fn create_heartbeat(device_id: &str) -> Result<Heartbeat> {
    let heartbeat = Heartbeat {
        device_id: device_id.to_string(),
        runner_version: "0.1.0".to_string(),
        status: RunnerStatus::Idle,
        current_task_id: None,
        current_task_progress: 0.0,
        system_resources: crate::models::SystemResources {
            cpu_percent: system::get_cpu_usage(),
            memory_used_mb: system::get_memory_used_mb(),
            memory_total_mb: system::get_memory_total_mb(),
            disk_used_gb: system::get_disk_used_gb(),
            disk_total_gb: system::get_disk_total_gb(),
        },
        capabilities: crate::models::Capabilities {
            supported_step_types: vec![
                crate::models::StepType::Python,
                crate::models::StepType::Binary,
                crate::models::StepType::Shell,
                crate::models::StepType::Api,
            ],
            has_oob_capture: false,
            has_gpu: false,
            gpu_model: None,
            oob_methods: vec![],
        },
        last_report: Utc::now().to_rfc3339(),
    };

    Ok(heartbeat)
}
