use common::prelude::{ServiceConfig, ServiceID};
use components::prelude::SmdbDataManager;

#[test]
fn test_new() {
    let mut data_manager = SmdbDataManager::new();

    let result = data_manager.get_all_services();
    assert!(result.is_ok());

    data_manager.close_connection().unwrap();
}

#[test]
fn test_get_all_services() {
    let mut data_manager = SmdbDataManager::new();

    let result = data_manager.get_all_services();
    assert!(result.is_ok());

    data_manager.close_connection().unwrap();
}

#[test]
fn test_get_service() {
    let data_manager = SmdbDataManager::new();

    let result = data_manager.get_service(&ServiceID::default());
    assert!(result.is_ok());
}

#[test]
fn test_check_all_service_dependencies() {
    let mut data_manager = SmdbDataManager::new();

    let result = data_manager.check_all_service_dependencies(&ServiceID::default());
    assert!(result.is_ok());

    data_manager.close_connection().unwrap();
}

#[test]
fn test_check_all_service_depends_on() {
    let mut data_manager = SmdbDataManager::new();

    let result = data_manager.check_all_service_depends_on(&ServiceID::default());
    assert!(result.is_ok());

    data_manager.close_connection().unwrap();
}

#[test]
fn test_get_all_service_dependencies() {
    let mut data_manager = SmdbDataManager::new();

    let result = data_manager.get_all_service_dependencies(&ServiceID::default());
    assert!(result.is_ok());

    data_manager.close_connection().unwrap();
}

#[test]
fn test_get_all_service_depends_on() {
    let mut data_manager = SmdbDataManager::new();

    let result = data_manager.get_all_service_depends_on(&ServiceID::default());
    assert!(result.is_ok());

    data_manager.close_connection().unwrap();
}

#[test]
fn test_create_service() {
    let mut data_manager = SmdbDataManager::new();

    let service_config = ServiceConfig::default();
    let result = data_manager.create_service(&service_config);
    assert!(result.is_ok());

    data_manager.close_connection().unwrap();
}

#[test]
fn test_check_service_exists() {
    let mut data_manager = SmdbDataManager::new();

    let result = data_manager.check_service_exists(&ServiceID::default());
    assert!(result.is_ok());

    data_manager.close_connection().unwrap();
}

#[test]
fn test_check_service_online() {
    let mut data_manager = SmdbDataManager::new();

    let result = data_manager.check_service_online(&ServiceID::default());
    assert!(result.is_ok());

    data_manager.close_connection().unwrap();
}

#[test]
fn test_delete_service() {
    let mut data_manager = SmdbDataManager::new();

    let result = data_manager.delete_service(&ServiceID::default());
    assert!(result.is_ok());

    data_manager.close_connection().unwrap();
}

#[test]
fn test_register_service() {
    let mut data_manager = SmdbDataManager::new();

    let service = ServiceID::default();
    let result = data_manager.register_service(&service);
    assert!(result.is_ok());

    data_manager.close_connection().unwrap();
}

#[test]
fn test_deregister_service() {
    let mut data_manager = SmdbDataManager::new();

    let service = ServiceID::default();
    let result = data_manager.deregister_service(&service);
    assert!(result.is_ok());

    data_manager.close_connection().unwrap();
}
