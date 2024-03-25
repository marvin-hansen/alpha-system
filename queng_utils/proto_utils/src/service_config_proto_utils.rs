use crate::endpoint_proto_utils::{endpoint_from_proto, endpoint_to_proto};
use crate::metric_config_proto_utils::{metric_config_from_proto, metric_config_to_proto};
use common::prelude::{ServiceConfig, ServiceID, ServiceType};
use proto_bindings::proto::ProtoServiceConfig;
use std::fmt::Error;

pub fn service_config_from_proto(proto: ProtoServiceConfig) -> Result<ServiceConfig, Error> {
    //
    let proto_svc_id = proto.svc_id;
    let svc_id = ServiceID::from(proto_svc_id);

    let proto_dependencies = proto.dependencies;
    let dependencies: Vec<ServiceID> = proto_dependencies.into_iter().map(|x| x.into()).collect();

    let proto_endpoint = proto
        .endpoint
        .expect("Failed to create endpoint from proto");

    let endpoint =
        endpoint_from_proto(proto_endpoint).expect("Failed to create endpoint from proto");

    let proto_metrics = proto.metrics.expect("Failed to create metrics from proto");
    let metrics =
        metric_config_from_proto(proto_metrics).expect("Failed to create metrics from proto");

    let proto_exposure = proto.exposure;
    let exposure = ServiceType::from(proto_exposure);

    Ok(ServiceConfig::new(
        svc_id,
        proto.name,
        proto.version as u8,
        proto.online,
        proto.description,
        proto.health_check_uri,
        proto.base_uri,
        dependencies,
        exposure,
        endpoint,
        metrics,
    ))
}

pub fn service_config_to_proto(service_config: ServiceConfig) -> Result<ProtoServiceConfig, Error> {
    //
    let proto_endpoint =
        endpoint_to_proto(service_config.endpoint()).expect("Failed to create endpoint from proto");

    let proto_metrics = metric_config_to_proto(service_config.metrics())
        .expect("Failed to create metrics from proto");

    let proto_dependencies = service_config
        .dependencies()
        .iter()
        .map(|x| x.to_owned() as i32)
        .collect::<Vec<i32>>();

    Ok(ProtoServiceConfig {
        svc_id: service_config.svc_id().to_owned() as i32,
        name: service_config.name().to_string(),
        version: service_config.version() as u32,
        online: service_config.online(),
        description: service_config.description().to_string(),
        health_check_uri: service_config.health_check_uri().to_string(),
        base_uri: service_config.base_uri().to_string(),
        endpoint: Some(proto_endpoint),
        metrics: Some(proto_metrics),
        dependencies: proto_dependencies,
        exposure: service_config.exposure().to_owned() as i32,
    })
}
