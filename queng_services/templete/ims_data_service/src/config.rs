use common_config::ServiceConfig;
use common_ims::IggyConfig;
use common_ims::{ExchangeID, IntegrationConfig};

pub fn ims_data_integration_config(exchange_id: ExchangeID) -> IntegrationConfig {
    shared_service_specs::ims_data_integration_config(exchange_id)
}

pub fn ims_data_iggy_config(exchange_id: ExchangeID) -> IggyConfig {
    shared_service_specs::ims_data_iggy_config(exchange_id)
}

pub fn ims_data_service_config(exchange_id: ExchangeID) -> ServiceConfig {
    shared_service_specs::ims_data_service_config(exchange_id)
}
