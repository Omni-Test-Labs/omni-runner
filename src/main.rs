mod api;
mod config;
mod models;
mod executor;
mod pipeline;
mod security;
mod utils;
mod system;
mod heartbeat;
mod tasks;

use anyhow::Result;
use tokio::signal;
use tracing::{info, warn, error};

use api::client::ApiClient;
use config::RunnerConfig;
use heartbeat::send_heartbeat;
use tasks::poll_and_execute_task;
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

