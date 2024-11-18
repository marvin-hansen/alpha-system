use crate::model::integration_message_config_type::MessageConfig;
use common_ims::{ExchangeID, IntegrationMessageConfig};

impl MessageConfig {
    // Convert from a IntegrationMessageConfigType to a MessageConfig
    pub fn from_integration_message_config_type(
        integration_message_config_type: IntegrationMessageConfig,
    ) -> Self {
        Self {
            id: integration_message_config_type.id().into(),
            name: integration_message_config_type.name().to_owned(),
            version: *integration_message_config_type.version() as i32,
            exchange_id: integration_message_config_type.exchange_id() as i32,
        }
    }
    // Convert from a MessageConfig to a IntegrationMessageConfigType
    pub fn to_integration_message_config_type(&self) -> IntegrationMessageConfig {
        IntegrationMessageConfig::new(
            self.id as u16,
            self.version as u16,
            ExchangeID::from(self.exchange_id),
        )
    }
}
