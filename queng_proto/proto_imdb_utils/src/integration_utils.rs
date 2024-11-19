use common_exchange::ExchangeID;
use common_ims::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};
use proto_imdb::proto::{ProtoIntegrationConfig, ProtoIntegrationMessageConfig};

/// Converts a `ProtoIntegrationConfig` into an `IntegrationConfig`.
///
/// # Arguments
///
/// * `proto` - The `ProtoIntegrationConfig` to convert
///
/// # Returns
///
/// An `IntegrationConfig` containing the converted configuration data
///
/// # Implementation Notes
///
/// This function:
/// 1. Converts `ProtoIntegrationConfig` fields to their domain counterparts
/// 2. Converts `ProtoIntegrationMessageConfig` fields to domain counterparts
/// 3. Provides a default `IntegrationMessageConfig` if the field is absent
///
/// # Errors
///
/// This function will not return an error, but will panic if:
/// * The `ProtoIntegrationConfig` is invalid
/// * The `ProtoIntegrationMessageConfig` conversion fails
/// * The `ExchangeID` cannot be converted from a `u32`
///
#[must_use]
pub fn integration_config_from_proto(proto: ProtoIntegrationConfig) -> IntegrationConfig {
    let exchange_id = ExchangeID::from(proto.exchange_id);

    let message_config = proto
        .integration_message_config
        .map(|msg_config| {
            IntegrationMessageConfig::new(
                msg_config.id as u16,
                msg_config.version as u16,
                exchange_id,
            )
        })
        .unwrap_or_else(|| IntegrationMessageConfig::new(1, 1, exchange_id));

    let integration_type = ImsIntegrationType::from(proto.ims_integration_type);

    IntegrationConfig::new(
        proto.integration_id,
        proto.integration_version as u16,
        integration_type,
        exchange_id,
        message_config,
    )
}

/// Converts an `IntegrationConfig` into a protobuf integration configuration.
///
/// # Arguments
///
/// * `config` - The `IntegrationConfig` to convert
///
/// # Returns
///
/// A `ProtoIntegrationConfig` containing the converted configuration data
///
/// # Implementation Notes
///
/// This function:
/// 1. Converts `IntegrationConfig` fields to their protobuf counterparts
/// 2. Converts `IntegrationMessageConfig` fields to protobuf counterparts
/// 3. Wraps `IntegrationMessageConfig` in a `Some` as it is an optional field in the protobuf
///
/// # Errors
///
/// This function will not return an error, but will panic if:
/// * The `IntegrationConfig` is invalid
/// * The `IntegrationMessageConfig` is invalid
/// * The `ExchangeID` cannot be converted to a `u32`
///
#[must_use]
pub fn integration_config_to_proto(config: IntegrationConfig) -> ProtoIntegrationConfig {
    let msg_config = config.integration_message_config();

    ProtoIntegrationConfig {
        integration_id: config.integration_id().to_string(),
        integration_version: u32::from(config.integration_version()),
        ims_integration_type: config.ims_integration_type() as u32,
        online: config.online(),
        exchange_id: config.exchange_id() as u32,
        integration_message_config: Some(ProtoIntegrationMessageConfig {
            id: u32::from(msg_config.id()),
            name: msg_config.name().to_string(),
            version: u32::from(*msg_config.version()),
            exchange_id: config.exchange_id() as u32,
        }),
    }
}
