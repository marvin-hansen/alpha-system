use common::types::config::memgraph_config::MemGraphConfig;

#[test]
fn test_new_connection() {
    let config = MemGraphConfig::new_connection(7687, Some("localhost".to_string()));

    assert_eq!(config.port(), 7687);
    assert_eq!(config.host(), &Some("localhost".to_string()));
}

#[test]
fn test_new_connection_with_authentication() {
    let config = MemGraphConfig::new_connection_with_authentication(
        7687,
        Some("localhost".to_string()),
        Some("username".to_string()),
        Some("password".to_string()),
    );

    assert_eq!(config.port(), 7687);
    assert_eq!(config.host(), &Some("localhost".to_string()));
    assert_eq!(config.username(), &Some("username".to_string()));
    assert_eq!(config.password(), &Some("password".to_string()));
}

#[test]
fn test_new_authentication() {
    let config = MemGraphConfig::new_authentication(
        Some("username".to_string()),
        Some("password".to_string()),
    );

    assert_eq!(config.username(), &Some("username".to_string()));
    assert_eq!(config.password(), &Some("password".to_string()));
}

#[test]
fn test_get_connect_params() {
    let config = MemGraphConfig::new_connection_with_authentication(
        7687,
        Some("localhost".to_string()),
        Some("username".to_string()),
        Some("password".to_string()),
    );

    assert_eq!(config.port(), 7687);
    assert_eq!(config.host(), &Some("localhost".to_string()));
    assert_eq!(config.username(), &Some("username".to_string()));
    assert_eq!(config.password(), &Some("password".to_string()));
    assert_eq!(config.client_name(), "rsmgclient/2.0.2");

    let connect_params = config.get_connect_params();

    assert_eq!(connect_params.host, Some("localhost".to_string()));
    assert_eq!(connect_params.port, 7687);
    assert_eq!(connect_params.username, Some("username".to_string()));
    assert_eq!(connect_params.password, Some("password".to_string()));
    assert_eq!(connect_params.client_name, "rsmgclient/2.0.2");
}

#[test]
fn test_default() {
    let config = MemGraphConfig::default();

    assert_eq!(config.port(), 7687);
    assert_eq!(config.host(), &None);
    assert_eq!(config.address(), &None);
    assert_eq!(config.username(), &None);
    assert_eq!(config.password(), &None);
    assert_eq!(config.client_name(), "rsmgclient/2.0.2");
}

#[test]
fn test_display() {
    let config = MemGraphConfig::new_connection_with_authentication(
        7687,
        Some("localhost".to_string()),
        Some("username".to_string()),
        Some("password".to_string()),
    );

    let expected = "MemGraphConfig { port: 7687, host: Some(\"localhost\"), address: None, username: Some(\"username\"), password: Some(\"password\"), client_name: \"rsmgclient/2.0.2\" }";
    let actual = format!("{:?}", config);
    assert_eq!(expected, actual);
}