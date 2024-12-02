use common_container::WaitStrategy;
use container_specs_iggy::iggy_container_config;

#[test]
fn test_iggy_container_config() {
    let container_config = iggy_container_config();
    assert_eq!(container_config.name(), "iggy");
    assert_eq!(container_config.image(), "iggyrs/iggy");
    assert_eq!(container_config.tag(), "0.4.84");
    assert_eq!(container_config.url(), "0.0.0.0");
    assert_eq!(container_config.connection_port(), 8090);
    assert!(container_config.additional_env_vars().is_none());
    assert!(container_config.platform().is_none());
    assert!(container_config.reuse_container());
    assert!(container_config.keep_configuration());
    assert_eq!(
        container_config.wait_strategy(),
        &WaitStrategy::WaitUntilConsoleOutputContains("Iggy server has started".to_string(), 30)
    );
}
