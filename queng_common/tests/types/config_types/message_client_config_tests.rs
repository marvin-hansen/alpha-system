use common::prelude::MessageClientConfig;

#[test]
fn test_new() {
    let config = MessageClientConfig::new(1, "client1".to_string());

    assert_eq!(config.id(), 1);
    assert_eq!(config.name(), "client1");
}

#[test]
fn test_default() {
    let default = MessageClientConfig::default();

    assert_eq!(default.id(), 0);
    assert_eq!(default.name(), "default_client");
}

#[test]
fn test_channel_getters() {
    let config = MessageClientConfig::new(1, "client1".to_string());

    assert_eq!(config.control_channel(), "client1-control");
    assert_eq!(config.data_channel(), "client1-data");
    assert_eq!(config.execution_channel(), "client1-execution");
}

#[test]
fn test_id() {
    let config = MessageClientConfig::new(1, "client".to_string());

    assert_eq!(config.id(), 1);
}


#[test]
fn test_name() {
    let config = MessageClientConfig::new(1, "client1".to_string());

    assert_eq!(config.name(), "client1");
}

#[test]
fn test_display() {
    let config = MessageClientConfig::new(1, "client1".to_string());

    let actual = format!("{}", config);
    let expected = "MessageClientConfig { id: 1, name: client1 }".to_string();

    assert_eq!(actual, expected);
}