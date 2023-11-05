use common::types::config::memgraph_config::MemGraphConfig;

#[test]
fn test_new_unsecure() {
    let config = MemGraphConfig::new_unsecure(7687, Some("localhost".to_string()));

    assert_eq!(config.port(), 7687);
    assert_eq!(config.host(), &Some("localhost".to_string()));
}

#[test]
fn test_new_with_authentication() {
    let config = MemGraphConfig::new_with_authentication(7687, Some("localhost".to_string()), Some("username".to_string()), Some("password".to_string()));

    assert_eq!(config.port(), 7687);
    assert_eq!(config.host(), &Some("localhost".to_string()));
    assert_eq!(config.username(), &Some("username".to_string()));
    assert_eq!(config.password(), &Some("password".to_string()));
}

#[test]
fn test_default() {
    let d = MemGraphConfig::default();

    assert_eq!(d.port(), 7687);
    assert_eq!(d.host(), &None);
    assert_eq!(d.address(), &None);
    assert_eq!(d.username(), &None);
    assert_eq!(d.password(), &None);
    assert_eq!(d.client_name(), "rsmgclient/2.0.2");
    // `SSLMode` cannot be formatted using `{:?}` because it doesn't implement `Debug`
    // assert_eq!(d.sslmode(), &SSLMode::Disable);
    assert_eq!(d.sslcert(), &None);
    assert_eq!(d.sslkey(), &None);
    // assert_eq!(d.trust_callback(), None);
    assert_eq!(d.lazy(), true);
    assert_eq!(d.autocommit(), false);
}