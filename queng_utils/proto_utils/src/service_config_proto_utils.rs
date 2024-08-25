use std::fmt::Error;

use common_config::prelude::{ServiceConfig, ServiceID};
use proto_bindings::proto::ProtoServiceConfig;

use crate::endpoint_proto_utils::{endpoint_from_proto, endpoint_to_proto};

/// Converts a `ProtoServiceConfig` into a `ServiceConfig`.
///
/// This function takes a `ProtoServiceConfig` and converts it into a `ServiceConfig` struct.
/// It extracts the necessary fields from the `ProtoServiceConfig` and constructs a new `ServiceConfig` with them.
///
/// # Errors
///
/// If the conversion of any field fails, an `std::fmt::Error` is returned.
///
pub fn service_config_from_proto(proto: ProtoServiceConfig) -> Result<ServiceConfig, Error> {
    //
    let proto_svc_id = proto.svc_id;
    let svc_id = ServiceID::from(proto_svc_id);

    let proto_dependencies = proto.dependencies;
    let dependencies: Vec<ServiceID> = proto_dependencies.into_iter().map(|x| x.into()).collect();

    let endpoint =
        endpoint_from_proto(proto.endpoint).expect("Failed to create endpoint from proto");

    Ok(ServiceConfig::new(
        svc_id,
        proto.name,
        proto.version,
        proto.online,
        proto.description,
        proto.health_check_uri,
        proto.base_uri,
        dependencies,
        endpoint,
    ))
}

///
/// Converts a `ServiceConfig` into a `ProtoServiceConfig`.
///
/// This function takes a `ServiceConfig` and converts it into a `ProtoServiceConfig` struct.
/// It extracts the necessary fields from the `ServiceConfig` and constructs a new `ProtoServiceConfig` with them.
///
/// # Errors
///
/// If the conversion of any field fails, an `std::fmt::Error` is returned.
///
pub fn service_config_to_proto(service_config: ServiceConfig) -> Result<ProtoServiceConfig, Error> {
    //
    let proto_endpoints = endpoint_to_proto(service_config.endpoints())
        .expect("Failed to create endpoint from proto");

    let proto_dependencies = service_config
        .dependencies()
        .iter()
        .map(|x| x.to_owned() as i32)
        .collect::<Vec<i32>>();

    Ok(ProtoServiceConfig {
        svc_id: service_config.svc_id().to_owned() as i32,
        name: service_config.name().to_string(),
        version: service_config.version(),
        online: service_config.online(),
        description: service_config.description().to_string(),
        health_check_uri: service_config.health_check_uri().to_string(),
        base_uri: service_config.base_uri().to_string(),
        endpoint: proto_endpoints,
        dependencies: proto_dependencies,
    })
}

pub fn service_config_collection_to_proto(
    service_configs: &[ServiceConfig],
) -> Result<Vec<ProtoServiceConfig>, Error> {
    let mut proto_configs = Vec::new();

    for service_config in service_configs {
        let p = service_config_to_proto(service_config.to_owned())
            .expect("Failed to convert Rust ServiceConfig to proto");

        proto_configs.push(p);
    }

    Ok(proto_configs)
}
