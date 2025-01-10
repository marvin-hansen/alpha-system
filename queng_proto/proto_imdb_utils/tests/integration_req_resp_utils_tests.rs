use common_exchange::ExchangeID;
use common_ims::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};
use proto_imdb::proto::*;
use proto_imdb_utils::*;

#[test]
fn test_get_create_integration_request() {
    let message_config = IntegrationMessageConfig::new(1, 2, ExchangeID::from(100));
    let config = IntegrationConfig::new(
        "valid_integration".to_string(),
        1,
        ImsIntegrationType::from(1_u16),
        ExchangeID::from(100),
        message_config,
    );

    let request = get_create_integration_request(config.clone());

    assert!(request.integration.is_some());
    let proto = request.integration.unwrap();
    assert_eq!(proto.integration_id, config.integration_id());
    assert_eq!(
        proto.integration_version,
        u32::from(config.integration_version())
    );
    assert_eq!(
        proto.ims_integration_type,
        config.ims_integration_type() as u32
    );
    assert_eq!(proto.exchange_id, config.exchange_id() as u32);
}

#[test]
fn test_get_count_integration_request() {
    let request = get_count_integration_request();
    // Assuming CountIntegrationRequest has no fields to validate
    // Verify that the request is correctly initialized
    // You can add more assertions if CountIntegrationRequest has fields in the future
    assert_eq!(request, CountIntegrationRequest {});
}

#[test]
fn test_get_check_if_integration_config_exists_request_with_valid_id() {
    let integration_id = "valid_integration_id".to_string();
    let request = get_check_if_integration_config_exists_request(integration_id.clone());
    assert_eq!(request.integration_id, integration_id);
}

#[test]
fn test_get_check_if_integration_config_exists_request_with_special_chars() {
    let integration_id = "test!@#$%^&*()_+-=[]{}|;:'\",.<>?/".to_string();
    let request = get_check_if_integration_config_exists_request(integration_id.clone());
    assert_eq!(request.integration_id, integration_id);
}

#[test]
fn test_get_check_if_integration_config_exists_request_with_empty_id() {
    let integration_id = String::new();
    let request = get_check_if_integration_config_exists_request(integration_id.clone());
    assert_eq!(request.integration_id, integration_id);
}

#[test]
fn test_get_check_if_integration_config_online_request_with_valid_id() {
    let integration_id = "online_integration_id".to_string();
    let request = get_check_if_integration_config_online_request(integration_id.clone());
    assert_eq!(request.integration_id, integration_id);
}

#[test]
fn test_get_check_if_integration_config_online_request_with_special_chars() {
    let integration_id = "online!@#$%^&*()_+-=[]{}|;:'\",.<>?/".to_string();
    let request = get_check_if_integration_config_online_request(integration_id.clone());
    assert_eq!(request.integration_id, integration_id);
}

#[test]
fn test_get_check_if_integration_config_online_request_with_empty_id() {
    let integration_id = String::new();
    let request = get_check_if_integration_config_online_request(integration_id.clone());
    assert_eq!(request.integration_id, integration_id);
}

#[test]
fn test_get_integration_request_with_valid_id() {
    let integration_id = "integration_123".to_string();
    let request = get_integration_request(&integration_id);
    assert_eq!(request.integration_id, integration_id);
}

#[test]
fn test_get_integration_request_with_special_chars() {
    let integration_id = "int!@#%&123".to_string();
    let request = get_integration_request(&integration_id);
    assert_eq!(request.integration_id, integration_id);
}

#[test]
fn test_get_integration_request_with_empty_id() {
    let integration_id = "";
    let request = get_integration_request(integration_id);
    assert_eq!(request.integration_id, "");
}

#[test]
fn test_get_all_integrations_by_exchange_request_with_valid_exchange_id() {
    let exchange_id = ExchangeID::BinanceSpot;
    let request = get_all_integrations_by_exchange_request(exchange_id);
    assert_eq!(request.exchange_id, exchange_id as i32);
}

#[test]
fn test_get_all_integrations_by_exchange_request_with_invalid_exchange_id() {
    let exchange_id = ExchangeID::from(i32::MAX);
    let request = get_all_integrations_by_exchange_request(exchange_id);
    assert_eq!(request.exchange_id, exchange_id as i32);
}

#[test]
fn test_get_all_integrations_request() {
    let request = get_all_integrations_request();
    // Assuming GetAllIntegrationsRequest has no fields to validate
    assert_eq!(request, GetAllIntegrationsRequest {});
}

#[test]
fn test_get_all_online_integrations_request() {
    let request = get_all_online_integrations_request();
    // Assuming GetAllOnlineIntegrationsRequest has no fields to validate
    assert_eq!(request, GetAllOnlineIntegrationsRequest {});
}

#[test]
fn test_get_all_offline_integrations_request() {
    let request = get_all_offline_integrations_request();
    // Assuming GetAllOfflineIntegrationsRequest has no fields to validate
    assert_eq!(request, GetAllOfflineIntegrationsRequest {});
}

#[test]
fn test_get_set_integration_online_request_with_valid_id() {
    let integration_id = "set_online_id".to_string();
    let request = get_set_integration_online_request(&integration_id);
    assert_eq!(request.integration_id, integration_id);
}

#[test]
fn test_get_set_integration_online_request_with_special_chars() {
    let integration_id = "set!@#online%&id".to_string();
    let request = get_set_integration_online_request(&integration_id);
    assert_eq!(request.integration_id, integration_id);
}

#[test]
fn test_get_set_integration_online_request_with_empty_id() {
    let integration_id = "";
    let request = get_set_integration_online_request(integration_id);
    assert_eq!(request.integration_id, "");
}

#[test]
fn test_get_set_integration_offline_request_with_valid_id() {
    let integration_id = "set_offline_id".to_string();
    let request = get_set_integration_offline_request(&integration_id);
    assert_eq!(request.integration_id, integration_id);
}

#[test]
fn test_get_set_integration_offline_request_with_special_chars() {
    let integration_id = "set!@#offline%&id".to_string();
    let request = get_set_integration_offline_request(&integration_id);
    assert_eq!(request.integration_id, integration_id);
}

#[test]
fn test_get_set_integration_offline_request_with_empty_id() {
    let integration_id = "";
    let request = get_set_integration_offline_request(integration_id);
    assert_eq!(request.integration_id, "");
}

/// Helper function to create a valid `IntegrationConfig` instance for testing.
fn create_valid_integration_config() -> IntegrationConfig {
    IntegrationConfig::new(
        "valid_integration_id".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::BinanceSpot,
        IntegrationMessageConfig::new(100, 12, ExchangeID::BinanceSpot),
    )
}

#[test]
fn test_get_update_integration_request_with_valid_config() {
    let config = create_valid_integration_config();
    let request = get_update_integration_request(config.clone());

    assert_eq!(request.integration_id, config.integration_id());

    assert!(request.integration.is_some());
    let proto = request.integration.unwrap();

    assert_eq!(proto.integration_id, config.integration_id());
    assert_eq!(
        proto.integration_version,
        u32::from(config.integration_version())
    );
    assert_eq!(
        proto.ims_integration_type,
        config.ims_integration_type() as u32
    );
    assert_eq!(proto.online, config.online());
    assert_eq!(proto.exchange_id, config.exchange_id() as u32);

    let msg_config_proto = proto.integration_message_config.unwrap();
    assert_eq!(
        msg_config_proto.id,
        u32::from(config.integration_message_config().id())
    );
    assert_eq!(
        msg_config_proto.name,
        config.integration_message_config().name()
    );
    assert_eq!(
        msg_config_proto.version,
        u32::from(*config.integration_message_config().version())
    );
    assert_eq!(msg_config_proto.exchange_id, config.exchange_id() as u32);
}

#[test]
fn test_get_delete_integration_request_with_valid_id() {
    let integration_id = "valid_delete_id";
    let request = get_delete_integration_request(integration_id);

    assert_eq!(request.integration_id, "valid_delete_id");
}

#[test]
fn test_get_delete_integration_request_with_special_characters() {
    let integration_id = "delete!@#id$%^&*()";
    let request = get_delete_integration_request(integration_id);

    assert_eq!(request.integration_id, "delete!@#id$%^&*()");
}

#[test]
fn test_get_delete_integration_request_with_empty_id() {
    let integration_id = "";
    let request = get_delete_integration_request(integration_id);

    assert_eq!(request.integration_id, "");
}

#[test]
fn test_get_delete_integration_request_with_unicode_characters() {
    let integration_id = "删除集成";
    let request = get_delete_integration_request(integration_id);

    assert_eq!(request.integration_id, "删除集成");
}

fn create_test_integration_config() -> ProtoIntegrationConfig {
    let message_config = ProtoIntegrationMessageConfig {
        id: 1,
        name: "test_integration".to_string(),
        version: 2,
        exchange_id: 100,
    };

    ProtoIntegrationConfig {
        integration_id: "test_integration".to_string(),
        integration_version: 2,
        ims_integration_type: 1,
        online: false,
        exchange_id: 100,
        integration_message_config: Some(message_config),
    }
}

#[test]
fn test_get_create_integration_response() {
    let response = get_create_integration_response();
    assert!(response.ok);
    assert!(response.error.is_none());
}

#[test]
fn test_get_count_integration_response() {
    let count = 42_u64;
    let response = get_count_integration_response(count);
    assert_eq!(response.count, count);
}

#[test]
fn test_get_check_if_integration_config_exists_response() {
    let exists_response = get_check_if_integration_config_exists_response(true);
    assert!(exists_response.exists);

    let not_exists_response = get_check_if_integration_config_exists_response(false);
    assert!(!not_exists_response.exists);
}

#[test]
fn test_get_check_if_integration_config_online_response() {
    let online_response = get_check_if_integration_config_online_response(true);
    assert!(online_response.online);

    let offline_response = get_check_if_integration_config_online_response(false);
    assert!(!offline_response.online);
}

#[test]
fn test_get_integration_config_response() {
    let config = create_test_integration_config();

    let some_response = get_integration_config_response(Some(config));
    assert!(some_response.integration.is_some());

    let none_response = get_integration_config_response(None);
    assert!(none_response.integration.is_none());
}

#[test]
fn test_get_all_integrations_response() {
    let configs = vec![
        create_test_integration_config(),
        create_test_integration_config(),
    ];
    let response = get_all_integrations_response(configs);
    assert_eq!(response.integrations.len(), 2);
}

#[test]
fn test_get_all_integrations_by_exchange_response() {
    let configs = vec![
        create_test_integration_config(),
        create_test_integration_config(),
    ];
    let response = get_all_integrations_by_exchange_response(configs);
    assert_eq!(response.integrations.len(), 2);
}

#[test]
fn test_get_all_online_integrations_response() {
    let configs = vec![
        create_test_integration_config(),
        create_test_integration_config(),
    ];
    let response = get_all_online_integrations_response(configs);
    assert_eq!(response.integrations.len(), 2);
}

#[test]
fn test_get_all_offline_integrations_response() {
    let configs = vec![
        create_test_integration_config(),
        create_test_integration_config(),
    ];
    let response = get_all_offline_integrations_response(configs);
    assert_eq!(response.integrations.len(), 2);
}

#[test]
fn test_get_set_integration_online_response() {
    let success_response = get_set_integration_online_response(true, None);
    assert!(success_response.ok);
    assert!(success_response.error.is_none());

    let error_response =
        get_set_integration_online_response(false, Some("Error message".to_string()));
    assert!(!error_response.ok);
    assert_eq!(error_response.error, Some("Error message".to_string()));
}

#[test]
fn test_get_set_integration_offline_response() {
    let success_response = get_set_integration_offline_response(true, None);
    assert!(success_response.ok);
    assert!(success_response.error.is_none());

    let error_response =
        get_set_integration_offline_response(false, Some("Error message".to_string()));
    assert!(!error_response.ok);
    assert_eq!(error_response.error, Some("Error message".to_string()));
}

#[test]
fn test_get_update_integration_response() {
    let success_response = get_update_integration_response(true, 1, None);
    assert!(success_response.ok);
    assert_eq!(success_response.nr_updated, 1);
    assert!(success_response.error.is_none());

    let error_response =
        get_update_integration_response(false, 0, Some("Update failed".to_string()));
    assert!(!error_response.ok);
    assert_eq!(error_response.nr_updated, 0);
    assert_eq!(error_response.error, Some("Update failed".to_string()));
}

#[test]
fn test_get_delete_integration_response() {
    let success_response = get_delete_integration_response(true, 1, None);
    assert!(success_response.ok);
    assert_eq!(success_response.nr_deleted, 1);
    assert!(success_response.error.is_none());

    let error_response =
        get_delete_integration_response(false, 0, Some("Delete failed".to_string()));
    assert!(!error_response.ok);
    assert_eq!(error_response.nr_deleted, 0);
    assert_eq!(error_response.error, Some("Delete failed".to_string()));
}
