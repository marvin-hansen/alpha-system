use common_config::prelude::ServiceID;
use imdb_specs::imdb_service_config;

#[test]
fn test_imdb_service_dependencies() {
    let config = imdb_service_config();
    let deps = config.dependencies();
    assert_eq!(deps.len(), 2);
    assert!(deps.contains(&ServiceID::SMDB));
    assert!(deps.contains(&ServiceID::DBGW));
}

#[test]
fn test_imdb_service_base_properties() {
    let config = imdb_service_config();

    assert_eq!(config.svc_id(), &ServiceID::IMDB);
    assert_eq!(config.name(), "imdb");
    assert_eq!(config.version(), 1);
    assert!(config.online());
    assert_eq!(
        config.cluster_uri(),
        "imdb-service.default.svc.cluster.local"
    );
    assert_eq!(
        config.health_check_uri(),
        "imdb-service.default.svc.cluster.local:7070/health"
    );
}
