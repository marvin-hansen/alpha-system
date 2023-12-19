use client_manager::ClientManager;

#[test]
fn test_add_client() {
    let mut manager = ClientManager::new();
    manager
        .add_client(1, "Client 1".to_string())
        .expect("Failed to add client");

    assert!(manager.check_client(1));
}

#[test]
fn test_get_client() {
    let mut manager = ClientManager::new();
    manager
        .add_client(1, "Client 1".to_string())
        .expect("Failed to add client");

    let client = manager.get_client(1).expect("Failed to get client");

    assert_eq!(client, &"Client 1".to_string());
}

#[test]
fn test_update_client() {
    let mut manager = ClientManager::new();
    manager
        .add_client(1, "Client 1".to_string())
        .expect("Failed to add client");

    manager.update_client(1, "Updated Client".to_string());

    let client = manager.get_client(1).expect("Failed to get client");

    assert_eq!(client, &"Updated Client".to_string());
}

#[test]
fn test_remove_client() {
    let mut manager = ClientManager::new();
    manager
        .add_client(1, "Client 1".to_string())
        .expect("Failed to add client");

    assert!(manager.check_client(1));

    manager.remove_client(1);

    assert!(!manager.check_client(1));
}
