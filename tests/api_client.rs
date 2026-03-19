use std::time::Duration;

use omni_runner::api::client::ApiClient;

#[test]
fn test_api_client_initialization() {
    let client = ApiClient::new(
        "http://localhost:8000".to_string(),
        "device-001".to_string(),
        None,
    ).unwrap();

    assert_eq!(client.base_url(), "http://localhost:8000");
}

#[test]
fn test_api_client_with_api_key() {
    let client = ApiClient::new(
        "http://localhost:8000".to_string(),
        "device-001".to_string(),
        Some("secret-key".to_string()),
    ).unwrap();

    assert_eq!(client.device_id(), "device-001");
    assert_eq!(client.api_key(), Some("secret-key"));
}

#[test]
fn test_api_client_without_api_key() {
    let client = ApiClient::new(
        "http://localhost:8000".to_string(),
        "device-001".to_string(),
        None,
    ).unwrap();

    assert_eq!(client.api_key(), None);
}

#[test]
fn test_api_client_with_special_characters() {
    let client = ApiClient::new(
        "http://test-device_001@example.com:8080/api".to_string(),
        "device-001".to_string(),
        None,
    );

    assert!(client.is_ok());
    if let Ok(client) = client {
        assert!(client.base_url().contains("test-device_001"));
    }
}

#[test]
fn test_api_client_custom_device_id() {
    let client = ApiClient::new(
        "http://localhost:8000".to_string(),
        "device-002".to_string(),
        Some("api-key".to_string()),
    ).unwrap();

    assert_eq!(client.device_id(), "device-002");
}

#[test]
fn test_api_client_long_base_url() {
    let long_url = "http://very.long.domain.name.with.many.parts.example.com:8080/api/v1".to_string();
    let client = ApiClient::new(
        long_url,
        "device-003".to_string(),
        None,
    );

    assert!(client.is_ok());
    if let Ok(client) = client {
        assert!(client.base_url().len() > 50);
    }
}

