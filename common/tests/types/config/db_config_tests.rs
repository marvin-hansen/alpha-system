use common::prelude::DBConfig;

#[test]
fn test_new_connection() {
    let config = DBConfig::new_connection(7687, Some("localhost".to_string()));

    assert_eq!(config.port(), 7687);
    assert_eq!(config.host(), &Some("localhost".to_string()));
}

#[test]
fn test_new_connection_with_authentication() {
    let config = DBConfig::new_connection_with_authentication(
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
    let config =
        DBConfig::new_authentication(Some("username".to_string()), Some("password".to_string()));

    assert_eq!(config.username(), &Some("username".to_string()));
    assert_eq!(config.password(), &Some("password".to_string()));
}

#[test]
fn test_default() {
    let config = DBConfig::default();

    assert_eq!(config.port(), 7687);
    assert_eq!(config.host(), &None);
    assert_eq!(config.username(), &None);
    assert_eq!(config.password(), &None);
    assert_eq!(config.client_name(), "rsmgclient/2.0.2");
}

#[test]
fn test_debug() {
    let config = DBConfig::new_connection_with_authentication(
        7687,
        Some("localhost".to_string()),
        Some("username".to_string()),
        Some("password".to_string()),
    );

    let expected = "DBConfig { port: 7687, host: Some(\"localhost\"), username: Some(\"username\"), password: Some(\"password\"), client_name: \"rsmgclient/2.0.2\" }";
    let actual = format!("{:?}", config);
    assert_eq!(expected, actual);
}

#[test]
fn test_display() {
    let config = DBConfig::new_connection_with_authentication(
        7687,
        Some("localhost".to_string()),
        Some("username".to_string()),
        Some("password".to_string()),
    );

    let expected = "DBConfig { port: 7687, host: Some(\"localhost\"), username: Some(\"username\"), password: Some(\"password\"), client_name: \"rsmgclient/2.0.2\" }";
    let actual = config.to_string();
    assert_eq!(expected, actual);
}
