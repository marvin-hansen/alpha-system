use common_exchange::prelude::ExchangeID;
use common_ims::prelude::{ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};
use proto_imdb::proto::{ProtoIntegrationConfig, ProtoIntegrationMessageConfig};

/// Converts a protobuf integration configuration into an `IntegrationConfig`.
///
/// # Parameters
///
/// * `proto` - The protobuf integration configuration to convert
///
/// # Returns
///
/// An `IntegrationConfig` containing the converted configuration data
///
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

/// Converts an `IntegrationConfig` into its protobuf representation.
///
/// # Parameters
///
/// * `config` - The integration configuration to convert
///
/// # Returns
///
/// A `ProtoIntegrationConfig` containing the converted configuration data
///
pub fn integration_config_to_proto(config: IntegrationConfig) -> ProtoIntegrationConfig {
    let msg_config = config.integration_message_config();

    ProtoIntegrationConfig {
        integration_id: config.integration_id().to_string(),
        integration_version: config.integration_version() as u32,
        ims_integration_type: config.ims_integration_type() as u32,
        online: config.online(),
        exchange_id: config.exchange_id() as u32,
        integration_message_config: Some(ProtoIntegrationMessageConfig {
            id: msg_config.id() as u32,
            name: msg_config.name().to_string(),
            version: *msg_config.version() as u32,
            exchange_id: config.exchange_id() as u32,
        }),
    }
}
