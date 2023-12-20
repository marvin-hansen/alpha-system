use client_manager::ClientManager;
use common::prelude::MessageClientConfig;

#[test]
fn test_add_client() {
    let mut manager = ClientManager::new();
    let config = MessageClientConfig::new(1, "Client 1".to_string());

    manager
        .add_client(1, config)
        .expect("Failed to add client");

    assert!(manager.check_client(1));
}

#[test]
fn test_get_client_control_channel() {
    let mut manager = ClientManager::new();
    let config = MessageClientConfig::new(1, "client1".to_string());
    manager.add_client(1, config.clone()).unwrap();

    let result = manager.get_client_control_channel(1);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), config.control_channel());

    let result = manager.get_client_control_channel(2);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "MessageClientConfigError: Client id 2 does not exist");
}

#[test]
fn test_get_client_data_channel() {
    let mut manager = ClientManager::new();
    let config = MessageClientConfig::new(1, "client1".to_string());
    manager.add_client(1, config.clone()).unwrap();

    let result = manager.get_client_data_channel(1);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), config.data_channel());

    let result = manager.get_client_data_channel(2);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "MessageClientConfigError: Client id 2 does not exist");
}


#[test]
fn test_get_client_execution_channel() {
    let mut manager = ClientManager::new();
    let config = MessageClientConfig::new(1, "client1".to_string());
    manager.add_client(1, config.clone()).unwrap();

    let result = manager.get_client_execution_channel(1);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), config.execution_channel());

    let result = manager.get_client_execution_channel(2);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "MessageClientConfigError: Client id 2 does not exist");
}


#[test]
fn test_get_client() {
    let mut manager = ClientManager::new();
    let config = MessageClientConfig::new(1, "Client 1".to_string());

    manager
        .add_client(1, config)
        .expect("Failed to add client");

    let client = manager.get_client_config(1).expect("Failed to get client");

    assert_eq!(client.name(), &"Client 1".to_string());
}

#[test]
fn test_update_client() {
    let mut manager = ClientManager::new();
    let config = MessageClientConfig::new(1, "Client 1".to_string());

    manager
        .add_client(1, config)
        .expect("Failed to add client");

    let client = manager.get_client_config(1).expect("Failed to get client");

    assert_eq!(client.name(), &"Client 1".to_string());

    let config_updated = MessageClientConfig::new(1, "Client 2".to_string());


    manager.update_client(1, config_updated);

    let client = manager.get_client_config(1).expect("Failed to get client");

    assert_eq!(client.name(), &"Client 2".to_string());
}

#[test]
fn test_remove_client() {
    let mut manager = ClientManager::new();
    let config = MessageClientConfig::new(1, "Client 1".to_string());

    manager
        .add_client(1, config)
        .expect("Failed to add client");

    assert!(manager.check_client(1));

    manager.remove_client(1);

    assert!(!manager.check_client(1));
}
