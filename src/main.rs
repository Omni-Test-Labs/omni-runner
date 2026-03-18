mod api;
mod config;
mod models;
mod executor;
mod pipeline;
mod security;
mod utils;

use anyhow::Result;
use tokio::signal;

use config::RunnerConfig;
use utils::logging::init_logging;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    println!("omni-runner v0.1.0 - Starting...");

    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config".to_string());
    let config = config::load_config(&config_path)?;

    println!("Loaded configuration for device: {}", config.device.device_id);
    println!("Server URL: {}", config.server.base_url);

    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("Received shutdown signal");
        },
        _ = run_main_loop(&config) => {
            println!("Main loop exited");
        },
    };

    Ok(())
}

async fn run_main_loop(config: &RunnerConfig) -> Result<()> {
    loop {
        let interval = tokio::time::Duration::from_secs(config.polling.interval_seconds);
        tokio::time::sleep(interval).await;
    }
}