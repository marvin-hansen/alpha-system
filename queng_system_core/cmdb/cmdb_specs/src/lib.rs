/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_config::{ServiceConfig, ServiceID};
use shared_service_specs::{default_grpc_service_endpoint, health_endpoint, metric_endpoint};

/// Constructs the configuration for the CMDB service.
///
/// This function generates a `ServiceConfig` struct with the following fields:
/// - `id`: The unique identifier for the service.
/// - `name`: The human-readable name of the service.
/// - `version`: The version of the service.
/// - `online`: A boolean indicating whether the service is online or not.
/// - `description`: A brief description of the service.
/// - `health_check_uri`: The URI for the health check endpoint of the service.
/// - `base_uri`: The base URI for the service.
/// - `dependencies`: A list of service IDs that this service depends on.
/// - `endpoints`: A list of `EndpointConfig` structs defining the endpoints of the service.
///
/// # Returns
/// A `ServiceConfig` instance with all the necessary settings for the CMDB service.
#[must_use]
pub fn cmdb_service_config() -> ServiceConfig {
    let id = ServiceID::CMDB;
    let name = "cmdb".to_string();
    let version = 1;
    let online = false;
    let description = "CMDB Manages configurations stored in the DB".to_string();
    let health_check_uri = "cmdb-service.default.svc.cluster.local:8080/health".to_string();
    let base_uri = "cmdb-service.default.svc.cluster.local".to_string();
    let dependencies = vec![ServiceID::DBGW, ServiceID::SMDB];
    let endpoints = vec![
        default_grpc_service_endpoint("CMDB Endpoint", 7070),
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
