use common_config::prelude::{ServiceConfig, ServiceID};
use common_ims::prelude::{
    ExchangeID, ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig,
};
use shared_service_specs::{health_endpoint, ims_endpoint, metric_endpoint};

/// Returns the configuration for the Binance data integration in the IMS system.
///
/// This function returns an `IntegrationConfig` instance with the specific settings for the Binance data integration.
/// It defines the integration ID, name, version, integration type, exchange ID and the message configuration for the integration.
///
/// # Returns
/// A `IntegrationConfig` instance with the specific settings for the Binance data integration in the IMS system.
///
pub fn ims_data_integration_binance_config() -> IntegrationConfig {
    IntegrationConfig::new(
        "binance".to_string(),
        1,
        ImsIntegrationType::Data,
        ExchangeID::Binance,
        IntegrationMessageConfig::new(1, 1, ExchangeID::Binance),
    )
}

/// Configures the service for Binance data in the IMS system.
///
/// This function generates a `ServiceConfig` tailored for managing data for a specific exchange within the IMS system.
/// It defines the service ID, name, version, online status, description, health check URI, base URI,
/// dependencies, exposure type, endpoint configuration, and metric configuration for the service.
///
/// # Returns
/// A `ServiceConfig` instance with all the necessary settings for the IMS service for the specified exchange.
///
/// # Fields
/// * `port` - The port number for the service.
/// * `id` - The ID of the service.
/// * `name` - The name of the service.
/// * `version` - The version of the service.
/// * `online` - The online status of the service.
/// * `description` - The description of the service.
/// * `health_check_uri` - The health check URI for the service.
/// * `base_uri` - The base URI for the service.
/// * `dependencies` - The dependencies of the service.
/// * `exposure` - The exposure type of the service.
/// * `endpoint` - The endpoint configuration for the service.
/// * `metrics` - The metric configuration for the service.
///
pub fn ims_data_binance_config() -> ServiceConfig {
    ims_service_config("Binance", ServiceID::ImsDataBinance)
}

fn ims_service_config(exchange_id: &str, service_id: ServiceID) -> ServiceConfig {
    let port = 7070;
    let id = service_id;
    let name = format!("ims-service-{}", exchange_id);
    let version = 1;
    let online = false;
    let description = format!("IMS controls streaming data for {} exchange", exchange_id);
    let health_check_uri = format!(
        "ims-data-service-{}.default.svc.cluster.local:{}/health",
        exchange_id, port
    );
    let base_uri = format!("ims-data-service-{}.default.svc.cluster.local", exchange_id);
    let dependencies = vec![ServiceID::SMDB];
    let endpoints = vec![
        ims_endpoint(exchange_id, port),
        metric_endpoint(),
        health_endpoint(),
    ];

    ServiceConfig::new(
        id,
        name,
        version,
        online,
        description,
        health_check_uri,
        base_uri,
        dependencies,
        endpoints,
    )
}
