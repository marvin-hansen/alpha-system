use crate::model::integration_config::UpdateIntegrationConfig;
use crate::model::integration_message_config_type::MessageConfig;
use common_ims::IntegrationConfig as CommonIntegrationConfig;

impl UpdateIntegrationConfig {
    pub fn from_common_integration_config(
        common_integration_config: CommonIntegrationConfig,
    ) -> UpdateIntegrationConfig {
        UpdateIntegrationConfig {
            integration_id: common_integration_config.integration_id().to_owned(),
            integration_version: common_integration_config.integration_version() as i32,
            ims_integration_type: common_integration_config.ims_integration_type() as i32,
            online: common_integration_config.online(),
            exchange_id: common_integration_config.exchange_id() as i32,
            integration_message_config: MessageConfig::from_integration_message_config_type(
                common_integration_config
                    .integration_message_config()
                    .clone(),
            ),
        }
    }
}
