use container_specs_iggy::iggy_container_config;

#[test]
fn test_iggy_container_config() {
    let container_config = iggy_container_config();
    assert_eq!(container_config.name(), "ci_iggy");
    assert_eq!(
        container_config.image(),
        "ghcr.io/marvin-hansen/ci_iggy/ci_iggy"
    );
    assert_eq!(container_config.tag(), "latest");
    assert_eq!(container_config.url(), "0.0.0.0");
    assert_eq!(container_config.connection_port(), 3000);
    assert!(container_config.additional_env_vars().is_none());
    assert!(container_config.platform().is_none());
    assert!(container_config.reuse_container());
    assert!(container_config.keep_configuration());
}
