// Request

use crate::integration_utils::integration_config_to_proto;
use common_exchange::ExchangeID;
use common_ims::IntegrationConfig;
use proto_imdb::proto::{
    CheckIfIntegrationConfigExistsRequest, CheckIfIntegrationConfigExistsResponse,
    CheckIfIntegrationConfigOnlineRequest, CheckIfIntegrationConfigOnlineResponse,
    CountIntegrationRequest, CountIntegrationResponse, CreateIntegrationRequest,
    CreateIntegrationResponse, DeleteIntegrationRequest, DeleteIntegrationResponse,
    GetAllIntegrationsByExchangeRequest, GetAllIntegrationsByExchangeResponse,
    GetAllIntegrationsRequest, GetAllIntegrationsResponse, GetAllOfflineIntegrationsRequest,
    GetAllOfflineIntegrationsResponse, GetAllOnlineIntegrationsRequest,
    GetAllOnlineIntegrationsResponse, GetIntegrationConfigRequest, GetIntegrationConfigResponse,
    ProtoIntegrationConfig, SetIntegrationOfflineRequest, SetIntegrationOfflineResponse,
    SetIntegrationOnlineRequest, SetIntegrationOnlineResponse, UpdateIntegrationRequest,
    UpdateIntegrationResponse,
};

#[must_use]
pub fn get_create_integration_request(
    integration_config: IntegrationConfig,
) -> CreateIntegrationRequest {
    CreateIntegrationRequest {
        integration: Some(integration_config_to_proto(integration_config)),
    }
}

#[must_use]
pub const fn get_count_integration_request() -> CountIntegrationRequest {
    CountIntegrationRequest {}
}

#[must_use]
pub const fn get_check_if_integration_config_exists_request(
    integration_id: String,
) -> CheckIfIntegrationConfigExistsRequest {
    CheckIfIntegrationConfigExistsRequest { integration_id }
}

#[must_use]
pub const fn get_check_if_integration_config_online_request(
    integration_id: String,
) -> CheckIfIntegrationConfigOnlineRequest {
    CheckIfIntegrationConfigOnlineRequest { integration_id }
}

#[must_use]
pub fn get_integration_request(integration_id: &str) -> GetIntegrationConfigRequest {
    GetIntegrationConfigRequest {
        integration_id: integration_id.to_string(),
    }
}

#[must_use]
pub const fn get_all_integrations_request() -> GetAllIntegrationsRequest {
    GetAllIntegrationsRequest {}
}

#[must_use]
pub const fn get_all_integrations_by_exchange_request(
    exchange_id: ExchangeID,
) -> GetAllIntegrationsByExchangeRequest {
    GetAllIntegrationsByExchangeRequest {
        exchange_id: exchange_id as i32,
    }
}

#[must_use]
pub const fn get_all_online_integrations_request() -> GetAllOnlineIntegrationsRequest {
    GetAllOnlineIntegrationsRequest {}
}

#[must_use]
pub const fn get_all_offline_integrations_request() -> GetAllOfflineIntegrationsRequest {
    GetAllOfflineIntegrationsRequest {}
}

#[must_use]
pub fn get_set_integration_online_request(integration_id: &str) -> SetIntegrationOnlineRequest {
    SetIntegrationOnlineRequest {
        integration_id: integration_id.to_string(),
    }
}

#[must_use]
pub fn get_set_integration_offline_request(integration_id: &str) -> SetIntegrationOfflineRequest {
    SetIntegrationOfflineRequest {
        integration_id: integration_id.to_string(),
    }
}

#[must_use]
pub fn get_update_integration_request(
    integration_config: IntegrationConfig,
) -> UpdateIntegrationRequest {
    UpdateIntegrationRequest {
        integration_id: integration_config.integration_id().to_string(),
        integration: Some(integration_config_to_proto(integration_config)),
    }
}

#[must_use]
pub fn get_delete_integration_request(integration_id: &str) -> DeleteIntegrationRequest {
    DeleteIntegrationRequest {
        integration_id: integration_id.to_string(),
    }
}

// Response

#[must_use]
pub const fn get_create_integration_response() -> CreateIntegrationResponse {
    CreateIntegrationResponse {
        ok: true,
        error: None,
    }
}

#[must_use]
pub const fn get_count_integration_response(count: u64) -> CountIntegrationResponse {
    CountIntegrationResponse { count }
}

#[must_use]
pub const fn get_check_if_integration_config_exists_response(
    exists: bool,
) -> CheckIfIntegrationConfigExistsResponse {
    CheckIfIntegrationConfigExistsResponse { exists }
}

#[must_use]
pub const fn get_check_if_integration_config_online_response(
    online: bool,
) -> CheckIfIntegrationConfigOnlineResponse {
    CheckIfIntegrationConfigOnlineResponse { online }
}

#[must_use]
pub const fn get_integration_config_response(
    integration_config: Option<ProtoIntegrationConfig>,
) -> GetIntegrationConfigResponse {
    GetIntegrationConfigResponse {
        integration: integration_config,
    }
}

#[must_use]
pub const fn get_all_integrations_response(
    integrations: Vec<ProtoIntegrationConfig>,
) -> GetAllIntegrationsResponse {
    GetAllIntegrationsResponse { integrations }
}

#[must_use]
pub const fn get_all_integrations_by_exchange_response(
    integrations: Vec<ProtoIntegrationConfig>,
) -> GetAllIntegrationsByExchangeResponse {
    GetAllIntegrationsByExchangeResponse { integrations }
}

#[must_use]
pub const fn get_all_online_integrations_response(
    integrations: Vec<ProtoIntegrationConfig>,
) -> GetAllOnlineIntegrationsResponse {
    GetAllOnlineIntegrationsResponse { integrations }
}

#[must_use]
pub const fn get_all_offline_integrations_response(
    integrations: Vec<ProtoIntegrationConfig>,
) -> GetAllOfflineIntegrationsResponse {
    GetAllOfflineIntegrationsResponse { integrations }
}

#[must_use]
pub const fn get_set_integration_online_response(
    ok: bool,
    error: Option<String>,
) -> SetIntegrationOnlineResponse {
    SetIntegrationOnlineResponse { ok, error }
}
#[must_use]
pub const fn get_set_integration_offline_response(
    ok: bool,
    error: Option<String>,
) -> SetIntegrationOfflineResponse {
    SetIntegrationOfflineResponse { ok, error }
}

#[must_use]
pub const fn get_update_integration_response(
    ok: bool,
    nr_updated: u32,
    error: Option<String>,
) -> UpdateIntegrationResponse {
    UpdateIntegrationResponse {
        ok,
        nr_updated,
        error,
    }
}

#[must_use]
pub const fn get_delete_integration_response(
    ok: bool,
    nr_deleted: u32,
    error: Option<String>,
) -> DeleteIntegrationResponse {
    DeleteIntegrationResponse {
        ok,
        nr_deleted,
        error,
    }
}
