use anyhow::{Context, Result};
use reqwest::Client as HttpClient;
use std::sync::Arc;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::models::{ExecutionResult, Heartbeat, TaskManifest};

/// HTTP client for communicating with omni-server
pub struct ApiClient {
    client: HttpClient,
    base_url: String,
    api_key: Option<String>,
    device_id: String,
}

impl ApiClient {
    pub fn new(
        base_url: String,
        device_id: String,
        api_key: Option<String>,
    ) -> Result<Self> {
        let client = HttpClient::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url,
            device_id,
            api_key,
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    async fn get_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        
        if let Some(ref key) = self.api_key {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", key)).unwrap(),
            );
        }
        
        headers
    }

    /// Poll for pending tasks from the server
    pub async fn poll_for_task(&self) -> Result<Option<TaskManifest>> {
        let url = format!("{}/api/v1/tasks?status=pending", self.base_url);
        
        debug!("Polling for tasks from {}", url);
        
        let response = self
            .client
            .get(&url)
            .headers(self.get_headers().await)
            .send()
            .await
            .context("Failed to poll for tasks")?;
        
        if response.status() == 404 {
            warn!("Tasks endpoint not found (404)");
            return Ok(None);
        }
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to poll tasks: {} - {}", status, body);
        }
        
        let tasks: Vec<TaskManifest> = response.json().await
            .context("Failed to parse tasks response")?;
        
        if tasks.is_empty() {
            return Ok(None);
        }
        
        let task = tasks.into_iter().next().unwrap();
        
        debug!("Received task: {}", task.task_id);
        
        Ok(Some(task))
    }

    /// Assign a task to this device
    pub async fn assign_task(&self, task_id: &str) -> Result<()> {
        let url = format!("{}/api/v1/tasks/{}/assign", self.base_url, task_id);
        
        info!("Assigning task {} to device {}", task_id, self.device_id);
        
        let response = self
            .client
            .put(&url)
            .headers(self.get_headers().await)
            .json(&serde_json::json!({
                "device_id": self.device_id
            }))
            .send()
            .await
            .context("Failed to assign task")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to assign task: {} - {}", status, body);
        }
        
        Ok(())
    }

    /// Send heartbeat to the server
    pub async fn send_heartbeat(&self, heartbeat: &Heartbeat) -> Result<()> {
        let url = format!("{}/api/v1/devices/{}/heartbeat", self.base_url, self.device_id);
        
        debug!("Sending heartbeat");
        
        let response = self
            .client
            .post(&url)
            .headers(self.get_headers().await)
            .json(heartbeat)
            .send()
            .await
            .context("Failed to send heartbeat")?;
        
        if !response.status().is_success() {
            let status = response.status();
            debug!("Heartbeat failed to send: {} (retrying later)", status);
            return Err(anyhow::anyhow!("Heartbeat failed: {}", status));
        }
        
        Ok(())
    }

    /// Report execution result to the server
    pub async fn report_result(&self, result: &ExecutionResult) -> Result<()> {
        let url = format!("{}/api/v1/tasks/{}/result", self.base_url, result.task_id);
        
        info!("Reporting result for task {}: {:?}", result.task_id, result.status);
        
        let response = self
            .client
            .post(&url)
            .headers(self.get_headers().await)
            .json(result)
            .send()
            .await
            .context("Failed to report result")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Failed to report result: {} - {}", status, body);
        }
        
        Ok(())
    }
}
