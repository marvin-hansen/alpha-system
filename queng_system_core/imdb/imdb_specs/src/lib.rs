/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_config::{ServiceConfig, ServiceID};
use shared_service_specs::{default_grpc_service_endpoint, health_endpoint, metric_endpoint};

#[must_use]
pub fn imdb_service_config() -> ServiceConfig {
    let id = ServiceID::IMDB;
    let name = "imdb".to_string();
    let version = 1;
    let online = false;
    let description = "IMDB gives access to integration services".to_string();
    let health_check_uri = "imdb-service.default.svc.cluster.local:7070/health".to_string();
    let base_uri = "imdb-service.default.svc.cluster.local".to_string();
    let dependencies = vec![ServiceID::SMDB, ServiceID::DBGW];
    let endpoints = vec![
        default_grpc_service_endpoint("imdb Endpoint", 7070),
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
