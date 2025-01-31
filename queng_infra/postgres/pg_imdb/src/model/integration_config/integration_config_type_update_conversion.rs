/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::model::integration_config::UpdateIntegrationConfig;
use crate::model::integration_message_config_type::MessageConfig;
use common_ims::IntegrationConfig as CommonIntegrationConfig;

impl UpdateIntegrationConfig {
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
}
