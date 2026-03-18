use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StepType {
    Python,
    Binary,
    Shell,
    Api,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Critical,
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Success,
    Failed,
    Crashed,
    Timeout,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetryPolicy {
    #[serde(default = "default_retries")]
    pub max_retries: u32,
    
    #[serde(default = "default_delay")]
    pub retry_delay_seconds: u64,
    
    #[serde(default = "default_backoff")]
    pub backoff_multiplier: f64,
}

fn default_retries() -> u32 { 0 }
fn default_delay() -> u64 { 5 }
fn default_backoff() -> f64 { 2.0 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkPolicy {
    #[serde(default)]
    pub allow_internet: bool,
    
    #[serde(default)]
    pub allowed_hosts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityPolicy {
    #[serde(default)]
    pub allow_sudo: bool,
    
    #[serde(default)]
    pub forbidden_cmds: Vec<String>,
    
    #[serde(default)]
    pub allowed_dirs: Vec<String>,
    
    #[serde(default)]
    pub network_policy: NetworkPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArtifactCollection {
    #[serde(default)]
    pub collect_logs: bool,
    
    #[serde(default)]
    pub collect_screenshots: bool,
    
    #[serde(default)]
    pub collect_profiles: bool,
    
    #[serde(default)]
    pub custom_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceBinding {
    pub device_id: String,
    pub device_type: String,
    #[serde(default)]
    pub oob_methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStep {
    pub step_id: String,
    pub order: u32,
    #[serde(rename = "type")]
    pub step_type: StepType,
    pub cmd: String,
    
    #[serde(default)]
    pub env: HashMap<String, String>,
    
    #[serde(default)]
    pub working_dir: Option<String>,
    
    #[serde(default = "default_must_pass")]
    pub must_pass: bool,
    
    #[serde(default)]
    pub depends_on: Vec<String>,
    
    #[serde(default)]
    pub always_run: bool,
    
    #[serde(default)]
    pub retry_policy: Option<RetryPolicy>,
    
    pub security_policy: SecurityPolicy,
    
    pub timeout_seconds: u64,
    
    #[serde(default)]
    pub artifact_collection: Option<ArtifactCollection>,
}

fn default_must_pass() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationHooks {
    #[serde(default)]
    pub on_start: Vec<String>,
    
    #[serde(default)]
    pub on_success: Vec<String>,
    
    #[serde(default)]
    pub on_failure: Vec<String>,
    
    #[serde(default)]
    pub on_timeout: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskManifest {
    pub schema_version: String,
    pub task_id: String,
    pub created_at: String,
    
    pub device_binding: DeviceBinding,
    
    pub priority: Priority,
    
    pub timeout_seconds: u64,
    
    pub pipeline: Vec<PipelineStep>,
    
    #[serde(default)]
    pub notification_hooks: Option<NotificationHooks>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    #[serde(default)]
    pub peak_memory_mb: u64,
    
    #[serde(default)]
    pub peak_cpu_percent: f64,
    
    #[serde(default)]
    pub disk_io_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
    
    #[serde(default)]
    pub stack_trace: Option<String>,
    
    #[serde(default)]
    pub context: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: String,
    
    #[serde(rename = "type")]
    pub status: TaskStatus,
    
    #[serde(default)]
    pub started_at: Option<String>,
    
    #[serde(default)]
    pub completed_at: Option<String>,
    
    #[serde(default)]
    pub duration_seconds: Option<f64>,
    
    #[serde(default)]
    pub exit_code: Option<i32>,
    
    #[serde(default)]
    pub signal: Option<String>,
    
    #[serde(default)]
    pub log_path: Option<String>,
    
    #[serde(default)]
    pub log_url: Option<String>,
    
    #[serde(default)]
    pub stdout_lines: Option<u64>,
    
    #[serde(default)]
    pub stderr_lines: Option<u64>,
    
    #[serde(default)]
    pub artifact_urls: Vec<String>,
    
    #[serde(default)]
    pub resource_usage: Option<ResourceUsage>,
    
    #[serde(default = "default_retry_count")]
    pub retry_count: u32,
    
    #[serde(default)]
    pub error: Option<ErrorInfo>,
    
    #[serde(default)]
    pub reason: Option<String>,
}

fn default_retry_count() -> u32 { 0 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceInfo {
    pub device_id: String,
    pub hostname: String,

    #[serde(default)]
    pub ip_address: Option<String>,

    #[serde(default)]
    pub os_version: Option<String>,

    pub runner_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub total_steps: u32,
    pub successful_steps: u32,
    pub failed_steps: u32,
    pub skipped_steps: u32,
    pub crashed_steps: u32,
    
    pub total_duration_seconds: f64,
    
    pub total_artifacts: u32,
    
    pub total_log_lines: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRca {
    pub status: String,
    pub root_cause: String,
    
    #[serde(default)]
    pub confidence: Option<f64>,
    
    #[serde(default)]
    pub analysis: Option<String>,
    
    #[serde(default)]
    pub related_issues: Vec<String>,
    
    #[serde(default)]
    pub next_actions: Vec<String>,
    
    #[serde(default)]
    pub model_used: Option<String>,
    
    #[serde(default)]
    pub analyzed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OobLog {
    pub method: String,
    pub captured_at: String,
    pub path: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemState {
    pub uptime_seconds: u64,
    pub load_average: Vec<f64>,
    pub disk_usage_percent: f64,
    pub memory_usage_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Forensics {
    #[serde(default)]
    pub oob_logs: Vec<OobLog>,

    #[serde(default)]
    pub system_state: SystemState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub schema_version: String,
    pub task_id: String,
    
    #[serde(rename = "type")]
    pub status: TaskStatus,
    
    pub started_at: String,
    
    #[serde(default)]
    pub completed_at: Option<String>,
    
    pub duration_seconds: f64,
    
    #[serde(default)]
    pub device_info: DeviceInfo,
    
    pub steps: Vec<StepResult>,
    
    pub summary: Summary,
    
    #[serde(default)]
    pub ai_rca: Option<AiRca>,
    
    #[serde(default)]
    pub forensics: Forensics,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RunnerStatus {
    Idle,
    Running,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemResources {
    pub cpu_percent: f64,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub disk_used_gb: u64,
    pub disk_total_gb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    #[serde(rename = "supported_step_types")]
    pub supported_step_types: Vec<StepType>,
    
    #[serde(rename = "has_oob_capture")]
    pub has_oob_capture: bool,
    
    #[serde(rename = "has_gpu")]
    pub has_gpu: bool,
    
    #[serde(default)]
    pub gpu_model: Option<String>,
    
    #[serde(default)]
    pub oob_methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    pub device_id: String,
    pub runner_version: String,
    
    #[serde(rename = "type")]
    pub status: RunnerStatus,
    
    #[serde(default)]
    pub current_task_id: Option<String>,
    
    #[serde(default)]
    pub current_task_progress: f64,
    
    pub system_resources: SystemResources,
    
    pub capabilities: Capabilities,
    
    pub last_report: String,
}
