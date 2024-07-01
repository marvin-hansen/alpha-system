use container_specs::api_proxy_container_config::api_proxy_container_config;

#[test]
fn test_api_proxy_container_config() {
    let config = api_proxy_container_config();

    assert_eq!(config.name(), "apiproxy");
    // The image tag changes to frequently during development.
    // Update and add tag test when stabilized
    assert_eq!(
        config.image(),
        "asia-northeast1-docker.pkg.dev/future-309012/image-repo/kaiko_proxy"
    );
    assert_eq!(config.url(), "0.0.0.0");
    assert_eq!(config.connection_port(), 7777);
    assert_eq!(config.platform(), Some("linux/amd64"));
    assert!(config.reuse_container());
    assert!(!config.reset_configuration());
}
