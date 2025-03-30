/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::model::integration_config::IntegrationConfig;
use crate::model::integration_message_config_type::MessageConfig;
use common_ims::{ExchangeID, ImsIntegrationType, IntegrationConfig as CommonIntegrationConfig};

impl IntegrationConfig {
    /// Converts a common integration configuration into an integration configuration.
    ///
    /// # Parameters
    ///
    /// * `common_integration_config`: The common integration configuration to convert from.
    ///
    /// # Returns
    ///
    /// A new `IntegrationConfig` instance containing the converted data.
    ///
    #[must_use]
    pub fn from_common_integration_config(
        common_integration_config: CommonIntegrationConfig,
    ) -> Self {
        Self {
            integration_id: common_integration_config.integration_id().to_owned(),
            integration_version: i32::from(common_integration_config.integration_version()),
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

    /// Converts this integration configuration into a common integration configuration.
    ///
    /// # Returns
    ///
    /// A new `CommonIntegrationConfig` instance containing the converted data.
    ///
    #[must_use]
    pub fn to_common_integration_config(&self) -> CommonIntegrationConfig {
        CommonIntegrationConfig::from(
            self.integration_id.clone(),
            self.integration_version as u16,
            ImsIntegrationType::from(self.ims_integration_type as u16),
            self.online,
            ExchangeID::from(self.exchange_id),
            self.integration_message_config
                .to_integration_message_config_type(),
        )
    }
}
