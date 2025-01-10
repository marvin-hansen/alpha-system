use common_config::{Endpoint, ProtocolType, ServiceConfig, ServiceID};
use common_ims::{ExchangeID, ImsIntegrationType, IntegrationConfig, IntegrationMessageConfig};
use common_ims::{IggyConfig, IggyUser};

/// Returns the configuration for the Binance data integration in the IMS system.
///
/// This function returns an `IntegrationConfig` instance with the specific settings for the Binance data integration.
/// It defines the integration ID, name, version, integration type, exchange ID and the message configuration for the integration.
///
/// # Returns
/// A `IntegrationConfig` instance with the specific settings for the Binance data integration in the IMS system.
///
pub fn ims_data_integration_config(exchange_id: ExchangeID) -> IntegrationConfig {
    IntegrationConfig::new(
        format!("{}-data", exchange_id),
        1,
        ImsIntegrationType::Data,
        exchange_id,
        IntegrationMessageConfig::new(1, 1, exchange_id),
    )
}

pub fn ims_data_iggy_config(exchange_id: ExchangeID) -> IggyConfig {
    IggyConfig::new(
        IggyUser::default(),
        "127.0.0.1:8090",
        exchange_id as u32,
        1,
        1,
        1,
        true,
    )
}

pub fn ims_data_service_config(exchange_id: ExchangeID) -> ServiceConfig {
    let port = 7070;
    let id = ServiceID::ImsData;
    let name = format!("ims-service-{}", exchange_id);
    let version = 1;
    let online = false;
    let description = format!("IMS streaming data for {exchange_id} exchange");
    let health_check_uri =
        format!("ims-data-service-{exchange_id}.default.svc.cluster.local:{port}/health");
    let base_uri = format!("ims-data-service-{exchange_id}.default.svc.cluster.local");
    let dependencies = vec![ServiceID::SMDB];
    let endpoints = vec![
        ims_endpoint(&exchange_id.to_string(), port),
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

#[must_use]
pub fn ims_endpoint(exchange_id: &str, port: u32) -> Endpoint {
    let endpoint_name = format!("{exchange_id}-ims-data-endpoint");
    let endpoint_version = 1;
    let endpoint_uri = "/".to_string();
    let endpoint_port = port;
    let endpoint_protocol = ProtocolType::GRPC;

    Endpoint::new(
        endpoint_name,
        endpoint_version,
        endpoint_uri,
        endpoint_port,
        endpoint_protocol,
    )
}

/// Creates a default gRPC service endpoint.
///
/// # Arguments
///
/// * `endpoint_name` - The name of the endpoint.
///
/// # Returns
///
/// A new `Endpoint` instance with the following fields:
/// - `name`: `endpoint_name`
/// - `version`: 1
/// - `uri`: "/"
/// - `port`: 7070
/// - `protocol`: `ProtocolType::GRPC`
///
#[must_use]
pub fn default_grpc_service_endpoint(endpoint_name: &str, endpoint_port: u32) -> Endpoint {
    let endpoint_name = endpoint_name.to_string();
    let endpoint_version = 1;
    let endpoint_uri = "/".to_string();
    let endpoint_protocol = ProtocolType::GRPC;

    Endpoint::new(
        endpoint_name,
        endpoint_version,
        endpoint_uri,
        endpoint_port,
        endpoint_protocol,
    )
}

/// Creates a default HTTP service endpoint.
///
/// # Arguments
///
/// * `endpoint_name` - The name of the endpoint.
/// * `endpoint_uri` - The URI of the endpoint.
///
/// # Returns
///
/// A new `Endpoint` instance with the following fields:
/// - `name`: `endpoint_name`
/// - `version`: 1
/// - `uri`: `endpoint_uri`
/// - `port`: 7070
/// - `protocol`: `ProtocolType::HTTP`
///
#[must_use]
pub fn default_http_service_endpoint(endpoint_name: &str, endpoint_uri: &str) -> Endpoint {
    let endpoint_name = endpoint_name.to_string();
    let endpoint_version = 1;
    let endpoint_uri = endpoint_uri.to_string();
    let endpoint_port = 7070;
    let endpoint_protocol = ProtocolType::HTTP;

    Endpoint::new(
        endpoint_name,
        endpoint_version,
        endpoint_uri,
        endpoint_port,
        endpoint_protocol,
    )
}

/// Constructs a default `Endpoint` for a metric service.
///
/// Returns an `Endpoint` instance with the following fields:
/// - `name`: "Metrics Endpoint"
/// - `version`: 1
/// - `uri`: "metrics"
/// - `port`: 8080
/// - `protocol`: `ProtocolType::HTTP`
///
#[must_use]
pub fn metric_endpoint() -> Endpoint {
    let endpoint_name = "Metrics Endpoint".to_string();
    let endpoint_version = 1;
    let endpoint_uri = "metrics".to_string();
    let endpoint_port = 8080;
    let endpoint_protocol = ProtocolType::HTTP;

    Endpoint::new(
        endpoint_name,
        endpoint_version,
        endpoint_uri,
        endpoint_port,
        endpoint_protocol,
    )
}

/// Creates a new `Endpoint` instance for the health check endpoint of a service.
///
/// # Returns
/// A new `Endpoint` instance with the following fields:
/// - `name`: "Health Endpoint"
/// - `version`: 1
/// - `uri`: "health"
/// - `port`: 8080
/// - `protocol`: `ProtocolType::HTTP`
///
#[must_use]
pub fn health_endpoint() -> Endpoint {
    let endpoint_name = "Health Endpoint".to_string();
    let endpoint_version = 1;
    let endpoint_uri = "health".to_string();
    let endpoint_port = 8080;
    let endpoint_protocol = ProtocolType::HTTP;

    Endpoint::new(
        endpoint_name,
        endpoint_version,
        endpoint_uri,
        endpoint_port,
        endpoint_protocol,
    )
}
