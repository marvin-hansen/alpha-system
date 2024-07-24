use common_config::prelude::{
    Endpoint, MetricConfig, ProtocolType, ServiceConfig, ServiceID, ServiceType,
};

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
    let exposure = ServiceType::ENDPOINT;
    let endpoint = get_endpoint(exchange_id, port);
    let metrics = get_metric_config();

    ServiceConfig::new(
        id,
        name,
        version,
        online,
        description,
        health_check_uri,
        base_uri,
        dependencies,
        exposure,
        endpoint,
        metrics,
    )
}

fn get_endpoint(exchange_id: &str, port: u32) -> Endpoint {
    let endpoint_name = format!("{}-ims-data-endpoint", exchange_id);
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

fn get_metric_config() -> MetricConfig {
    let metric_host = "0.0.0.0".to_string();
    let metric_uri = "metrics".to_string();
    let metric_port = 8080;

    MetricConfig::new(metric_uri, metric_host, metric_port)
}
