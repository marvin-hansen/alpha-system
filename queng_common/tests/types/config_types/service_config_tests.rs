use common::prelude::{Endpoint, MetricConfig, ServiceConfig, ServiceID, ServiceType};
use proto::binding::{ProtoEndpoint, ProtoMetricConfig, ProtoServiceConfig};

#[test]
fn test_new() {
    let id = ServiceID::SMDB;
    let name = "name".to_string();
    let version = 1;
    let online = true;
    let description = "description".to_string();
    let health_check_uri = "health_check_uri".to_string();
    let base_uri = "base_uri".to_string();
    let dependencies = vec![ServiceID::DBGW];
    let exposure = ServiceType::default();
    let endpoint = Endpoint::default();
    let metrics = MetricConfig::default();

    let service_config = ServiceConfig::new(
        id,
        name,
        version,
        online,
        description,
        health_check_uri,
        base_uri,
        dependencies,
        exposure,
        endpoint,
        metrics,
    );

    assert_eq!(service_config.svc_id(), &ServiceID::SMDB);
    assert_eq!(service_config.name(), String::from("name"));
    assert_eq!(service_config.version(), 1);
    assert!(service_config.online());
    assert_eq!(service_config.description(), String::from("description"));
    assert_eq!(
        service_config.health_check_uri(),
        String::from("health_check_uri")
    );
    assert_eq!(service_config.base_uri(), String::from("base_uri"));
    assert_eq!(
        service_config.dependencies().len(),
        vec![ServiceID::DBGW].len()
    );
    assert_eq!(service_config.exposure(), &ServiceType::default());
    assert_eq!(service_config.endpoint(), Endpoint::default());
}

#[test]
fn test_from_proto() {
    let proto = ProtoServiceConfig {
        svc_id: 1,
        name: "Test Service".to_string(),
        version: 1,
        online: true,
        description: "Test description".to_string(),
        health_check_uri: "/health".to_string(),
        base_uri: "http://localhost:8080".to_string(),
        dependencies: vec![2, 3],
        exposure: 1,
        endpoint: Some(ProtoEndpoint::default()),
        metrics: Some(ProtoMetricConfig::default()),
    };

    let config = ServiceConfig::from_proto(proto).unwrap();

    assert_eq!(config.svc_id(), &ServiceID::SMDB);
    assert_eq!(config.name(), "Test Service");
    assert_eq!(config.version(), 1);
    assert!(config.online());
    assert_eq!(config.description(), "Test description");
    assert_eq!(config.health_check_uri(), "/health");
    assert_eq!(config.base_uri(), "http://localhost:8080");
    assert_eq!(
        config.dependencies(),
        &vec![ServiceID::CMDB, ServiceID::DBGW]
    );
    assert_eq!(config.exposure(), &ServiceType::ENDPOINT);
}

#[test]
fn test_to_proto() {
    let config = ServiceConfig::new(
        ServiceID::SMDB,
        "Test Service".to_string(),
        1,
        true,
        "Test description".to_string(),
        "/health".to_string(),
        "http://localhost:8080".to_string(),
        vec![ServiceID::CMDB, ServiceID::DBGW],
        ServiceType::ENDPOINT,
        Endpoint::default(),
        MetricConfig::default(),
    );

    let proto = config.to_proto().unwrap();

    assert_eq!(proto.svc_id, 1);
    assert_eq!(proto.name, "Test Service");
    assert_eq!(proto.version, 1);
    assert!(proto.online);
    assert_eq!(proto.description, "Test description");
    assert_eq!(proto.health_check_uri, "/health");
    assert_eq!(proto.base_uri, "http://localhost:8080");
    assert_eq!(proto.dependencies, vec![2, 3]);
    assert_eq!(proto.exposure, 1);
}

#[test]
fn test_to_json() {
    let id = ServiceID::SMDB;
    let name = "name".to_string();
    let version = 1;
    let online = true;
    let description = "description".to_string();
    let health_check_uri = "health_check_uri".to_string();
    let base_uri = "base_uri".to_string();
    let dependencies = vec![ServiceID::DBGW];
    let exposure = ServiceType::default();
    let endpoint = Endpoint::default();
    let metrics = MetricConfig::default();

    let service_config = ServiceConfig::new(
        id,
        name,
        version,
        online,
        description,
        health_check_uri,
        base_uri,
        dependencies,
        exposure,
        endpoint,
        metrics,
    );

    let actual = service_config.to_json().unwrap();
    let expected = "{\"id\":null,\"svc_id\":\"SMDB\",\"name\":\"name\",\"version\":1,\"online\":true,\"description\":\"description\",\"health_check_uri\":\"health_check_uri\",\"base_uri\":\"base_uri\",\"dependencies\":[\"DBGW\"],\"exposure\":\"ENDPOINT\",\"endpoint\":{\"name\":\"\",\"version\":0,\"description\":\"\",\"uri\":\"\",\"port\":0,\"protocol\":\"NullVal\",\"encoding\":\"NullVal\"},\"metrics\":{\"metric_uri\":\"metrics\",\"metric_host\":\"127.0.0.1\",\"metric_port\":8080}}";
    assert_eq!(expected, actual)
}

#[test]
fn test_default() {
    let service_config = ServiceConfig::default();

    assert_eq!(service_config.svc_id(), &ServiceID::Default);
    assert_eq!(service_config.name(), &String::from(""));
    assert_eq!(service_config.version(), 0);
    assert!(!service_config.online());
    assert_eq!(service_config.description(), &String::from(""));
    assert_eq!(service_config.health_check_uri(), &String::from(""));
    assert_eq!(service_config.base_uri(), &String::from(""));
    assert_eq!(service_config.dependencies(), &Vec::new());
    assert_eq!(service_config.exposure(), &ServiceType::default());
    assert_eq!(service_config.endpoint(), Endpoint::default());
}

#[test]
fn test_display() {
    let id = ServiceID::SMDB;
    let name = "SMDB".to_string();
    let version = 1;
    let online = true;
    let description = "description".to_string();
    let health_check_uri = "health_check_uri".to_string();
    let base_uri = "base_uri".to_string();
    let dependencies = vec![ServiceID::DBGW];
    let exposure = ServiceType::default();
    let endpoint = Endpoint::default();
    let metrics = MetricConfig::default();

    let service_config = ServiceConfig::new(
        id,
        name,
        version,
        online,
        description,
        health_check_uri,
        base_uri,
        dependencies,
        exposure,
        endpoint,
        metrics,
    );

    let expected = "ServiceConfig { svc_id: SMDB, name: SMDB, version: 1, online: true, description: description, health_check_uri: health_check_uri, base_uri: base_uri, dependencies: [DBGW], exposure: ENDPOINT, endpoint: name: ,  version: 0,  port: 0,  description: ,  uri: ,  protocol: NullVal,  encoding: NullVal metrics: metric_uri: metrics,  metric_host: 127.0.0.1,  metric_port: 8080 }".to_string();
    let actual = service_config.to_string();
    assert_eq!(actual, expected);
}
