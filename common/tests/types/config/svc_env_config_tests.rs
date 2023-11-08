use common::prelude::{ServiceID, SvcEnvConfig};

#[test]
fn test_svc_env_config_new() {
    let service_id = ServiceID::CMDB;
    let cluster_host = "127.0.0.1".to_string();
    let local_host = "127.0.0.1".to_string();
    let port = "8080".to_string();
    let config = SvcEnvConfig::new(service_id, cluster_host, local_host, port);

    assert_eq!(config.service_id(), ServiceID::CMDB);
}

#[test]
fn test_svc_env_config_cluster_host() {
    let service_id = ServiceID::CMDB;
    let cluster_host = "127.0.0.1".to_string();
    let local_host = "127.0.0.1".to_string();
    let port = "8080".to_string();
    let config = SvcEnvConfig::new(service_id, cluster_host, local_host, port);

    assert_eq!(config.service_id(), ServiceID::CMDB);
    assert_eq!(config.cluster_host(), "127.0.0.1".to_string());
}

#[test]
fn test_svc_env_config_local_host() {
    let service_id = ServiceID::CMDB;
    let cluster_host = "127.0.0.1".to_string();
    let local_host = "127.0.0.1".to_string();
    let port = "8080".to_string();
    let config = SvcEnvConfig::new(service_id, cluster_host, local_host, port);

    assert_eq!(config.service_id(), ServiceID::CMDB);
    assert_eq!(config.cluster_host(), "127.0.0.1".to_string());
    assert_eq!(config.local_host(), "127.0.0.1".to_string());
}

#[test]
fn test_svc_env_config_port() {
    let service_id = ServiceID::CMDB;
    let cluster_host = "127.0.0.1".to_string();
    let local_host = "127.0.0.1".to_string();
    let port = "8080".to_string();
    let config = SvcEnvConfig::new(service_id, cluster_host, local_host, port);

    assert_eq!(config.service_id(), ServiceID::CMDB);
    assert_eq!(config.cluster_host(), "127.0.0.1".to_string());
    assert_eq!(config.local_host(), "127.0.0.1".to_string());
    assert_eq!(config.port(), "8080".to_string());
}

#[test]
fn test_svc_env_config_display() {
    let service_id = ServiceID::CMDB;
    let cluster_host = "127.0.0.1".to_string();
    let local_host = "127.0.0.1".to_string();
    let port = "8080".to_string();
    let config = SvcEnvConfig::new(service_id, cluster_host, local_host, port);
    let expected = "ServiceConfig { service_id: CMDB, cluster_host: \"127.0.0.1\", local_host: \"127.0.0.1\", port: \"8080\" }";
    assert_eq!(config.to_string(), expected);
}