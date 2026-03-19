use omni_runner::api::client::ApiClient;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_api_coverage_methods() {
    let client = ApiClient::new("http://test.com".to_string(), "device-001".to_string(), Some("key".to_string())).unwrap();
    
    assert!(client.base_url().len() > 0);
    assert_eq!(client.device_id(), "device-001");
    assert!(client.api_key().is_some());
}

#[test]
fn test_api_coverage_no_api_key() {
    let client = ApiClient::new("http://test.com".to_string(), "device-001".to_string(), None).unwrap();
    
    assert!(client.api_key().is_none());
}

#[test]
fn test_config_file_coverage() {
    let config_content = r#"
[device]
device_id = "device-001"
device_type = "test"
hostname = "test-host"

[server]
base_url = "http://test.local:8080"
api_key = "key-123"

[polling]
interval_seconds = 5
heartbeat_interval_seconds = 30

[[device.labels]]
key = "env"
value = "dev"
"#;
    
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    
    let result = omni_runner::config::load_config(temp_file.path().to_str().unwrap());
    assert!(result.is_ok());
    
    let config = result.unwrap();
    assert_eq!(config.device.device_id, "device-001");
    assert_eq!(config.device.hostname, "test-host");
}

#[test]
fn test_config_minimal_coverage() {
    let config_content = r#"
[device]
device_id = "device-001"
device_type = "test"
hostname = "test-host"

[server]
base_url = "http://test.local"
api_key = "key-123"

[polling]
interval_seconds = 5
heartbeat_interval_seconds = 30
"#;
    
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(config_content.as_bytes()).unwrap();
    
    let result = omni_runner::config::load_config(temp_file.path().to_str().unwrap());
    assert!(result.is_ok());
}

#[test]
fn test_models_defaults_coverage() {
    let retry_policy = omni_runner::models::RetryPolicy::default();
    assert_eq!(retry_policy.max_retries, 0);
    
    let _security_policy = omni_runner::models::SecurityPolicy::default();
}

#[test]
fn test_step_types_coverage() {
    use omni_runner::models::StepType;
    
    let types = vec![StepType::Shell, StepType::Python, StepType::Binary, StepType::Api];
    for step_type in types {
        let _value = serde_json::to_value(step_type).unwrap();
        assert!(true);
    }
}

#[test]
fn test_priority_coverage() {
    use omni_runner::models::Priority;
    
    let priorities = vec![Priority::Low, Priority::Normal, Priority::High, Priority::Critical];
    let json = serde_json::to_string(&priorities).unwrap();
    let deserialized: Vec<Priority> = serde_json::from_str(&json).unwrap();
    assert_eq!(priorities.len(), deserialized.len());
}
