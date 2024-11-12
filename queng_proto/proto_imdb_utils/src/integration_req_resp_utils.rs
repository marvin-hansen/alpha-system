// Request

use crate::integration_utils::integration_config_to_proto;
use common_ims::prelude::IntegrationConfig;
use proto_imdb::proto::*;

pub fn get_create_integration_request(
    integration_config: IntegrationConfig,
) -> CreateIntegrationRequest {
    CreateIntegrationRequest {
        integration: Some(integration_config_to_proto(integration_config)),
    }
}

pub fn get_count_integration_request() -> CountIntegrationRequest {
    CountIntegrationRequest {}
}

pub fn get_check_if_integration_config_exists_request(
    integration_id: String,
) -> CheckIfIntegrationConfigExistsRequest {
    CheckIfIntegrationConfigExistsRequest { integration_id }
}

pub fn get_delete_integration_request(integration_id: &str) -> DeleteIntegrationRequest {
    DeleteIntegrationRequest {
        integration_id: integration_id.to_string(),
    }
}

pub fn get_integration_request(integration_id: &str) -> GetIntegrationConfigRequest {
    GetIntegrationConfigRequest {
        integration_id: integration_id.to_string(),
    }
}

pub fn get_all_integrations_request() -> GetAllIntegrationsRequest {
    GetAllIntegrationsRequest {}
}

pub fn get_update_integration_request(
    integration_config: IntegrationConfig,
) -> UpdateIntegrationRequest {
    UpdateIntegrationRequest {
        integration_id: integration_config.integration_id().to_string(),
        integration: Some(integration_config_to_proto(integration_config)),
    }
}

// Response
