/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use container_specs_kaiko::api_proxy_container_config;

#[test]
fn test_api_proxy_container_config() {
    let config = api_proxy_container_config();

    assert_eq!(config.name(), "apiproxy");
    // The image tag changes to frequently during development.
    // Update and add tag test when stabilized
    assert_eq!(config.image(), "index.docker.io/hansenmarvin/api_proxy");
    assert_eq!(config.url(), "0.0.0.0");
    assert_eq!(config.connection_port(), 7777);
    assert!(config.platform().is_none());
    assert!(config.reuse_container());
    assert!(config.keep_configuration());
}
