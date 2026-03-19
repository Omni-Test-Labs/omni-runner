#[test]
fn test_init_logging_does_not_panic() {
    // Test that init_logging can be called without panicking
    // Note: This may affect global state, so we only verify it doesn't crash
    // In a real integration test, you'd reset tracing between tests
    #[cfg(feature = "logging")]
    {
        omni_runner::utils::logging::init_logging();
        assert!(true);
    }
}

