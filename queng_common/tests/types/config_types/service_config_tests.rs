use common::prelude::{Endpoint, MetricConfig, ServiceConfig, ServiceID, ServiceType};

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
