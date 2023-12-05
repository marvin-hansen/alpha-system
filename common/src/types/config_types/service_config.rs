use dbgw_proto::bindings::ProtoServiceConfig;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};
use surrealdb::sql::Thing;

use crate::prelude::{Endpoint, MetricConfig, ServiceID, ServiceType};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct ServiceConfig {
    // DB ID
    id: Option<Thing>,
    /// Unique Service ID.
    svc_id: ServiceID,
    /// Service name.
    name: String,
    /// Service version.
    version: u8,
    /// Whether the service is online.
    online: bool,
    /// Service description.
    description: String,
    /// Health check URI.
    health_check_uri: String,
    /// Base URI.
    base_uri: String,
    /// Service dependencies.
    dependencies: Vec<ServiceID>,
    /// Service exposure type.
    exposure: ServiceType,
    /// Service endpoint.
    endpoint: Endpoint,
    /// Service metrics
    metrics: MetricConfig,
}

impl ServiceConfig {
    /// Creates a new `ServiceConfig` instance.
    ///
    /// # Arguments
    ///
    /// * `id` - Service ID.
    /// * `name` - Service name.
    /// * `version` - Service version.
    /// * `online` - Whether the service is online.
    /// * `description` - Service description.
    /// * `health_check_uri` - Health check URI.
    /// * `base_uri` - Base URI.
    /// * `dependencies` - Service dependencies.
    /// * `exposure` - Service exposure type.
    /// * `endpoint` - Service endpoint.
    // https://rust-lang.github.io/rust-clippy/master/index.html#/too_many_arguments
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        svc_id: ServiceID,
        name: String,
        version: u8,
        online: bool,
        description: String,
        health_check_uri: String,
        base_uri: String,
        dependencies: Vec<ServiceID>,
        exposure: ServiceType,
        endpoint: Endpoint,
        metrics: MetricConfig,
    ) -> Self {
        Self {
            id: None,
            svc_id,
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
        }
    }
}

impl ServiceConfig {
    pub fn from_proto(proto: ProtoServiceConfig) -> Result<ServiceConfig, Error> {
        let proto_svc_id = proto.svc_id;
        let svc_id = ServiceID::from(proto_svc_id);

        let proto_dependencies = proto.dependencies;
        let dependencies: Vec<ServiceID> =
            proto_dependencies.into_iter().map(|x| x.into()).collect();

        let proto_endpoint = proto
            .endpoint
            .expect("Failed to create endpoint from proto");

        let endpoint =
            Endpoint::from_proto(proto_endpoint).expect("Failed to create endpoint from proto");

        let proto_metrics = proto.metrics.expect("Failed to create metrics from proto");
        let metrics =
            MetricConfig::from_proto(proto_metrics).expect("Failed to create metrics from proto");

        let proto_exposure = proto.exposure;
        let exposure = ServiceType::from(proto_exposure);

        Ok(ServiceConfig {
            id: None,
            svc_id,
            name: proto.name.into(),
            version: proto.version as u8,
            online: proto.online,
            description: proto.description,
            health_check_uri: proto.health_check_uri,
            base_uri: proto.base_uri,
            dependencies,
            exposure,
            endpoint,
            metrics,
        })
    }

    pub fn to_proto(&self) -> Result<ProtoServiceConfig, Error> {
        let proto_dependencies = self
            .dependencies
            .iter()
            .map(|x| x.to_owned() as i32)
            .collect::<Vec<i32>>();
        let proto_endpoint = self
            .endpoint
            .to_proto()
            .expect("Failed to create endpoint from proto");
        let proto_metrics = self
            .metrics
            .to_proto()
            .expect("Failed to create metrics from proto");

        Ok(ProtoServiceConfig {
            svc_id: self.svc_id as i32,
            name: self.name.clone(),
            version: self.version as u32,
            online: self.online,
            description: self.description.clone(),
            health_check_uri: self.health_check_uri.clone(),
            base_uri: self.base_uri.clone(),
            dependencies: proto_dependencies,
            exposure: self.exposure as i32,
            endpoint: Some(proto_endpoint),
            metrics: Some(proto_metrics),
        })
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        // https://github.com/serde-rs/json
        let json = serde_json::to_string(&self).expect("Failed to serialize ServiceConfig to JSON");

        Ok(json)
    }
}

impl ServiceConfig {
    /// Returns the service ID.
    pub fn svc_id(&self) -> &ServiceID {
        &self.svc_id
    }
    /// Returns the service name.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the service version.
    pub fn version(&self) -> u8 {
        self.version
    }
    /// Returns whether the service is online.
    pub fn online(&self) -> bool {
        self.online
    }
    /// Returns the service description.
    pub fn description(&self) -> &str {
        &self.description
    }
    /// Returns the health check URI.
    pub fn health_check_uri(&self) -> &str {
        &self.health_check_uri
    }
    /// Returns the base URI.
    pub fn base_uri(&self) -> &str {
        &self.base_uri
    }
    /// Returns the service dependencies.
    pub fn dependencies(&self) -> &Vec<ServiceID> {
        &self.dependencies
    }
    /// Returns the service exposure type.
    pub fn exposure(&self) -> &ServiceType {
        &self.exposure
    }
    /// Returns the service endpoint.
    pub fn endpoint(&self) -> Endpoint {
        self.endpoint.to_owned()
    }
    pub fn metrics(&self) -> &MetricConfig {
        &self.metrics
    }
}

impl Display for ServiceConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "ServiceConfig {{ svc_id: {}, name: {}, version: {}, online: {}, description: {}, health_check_uri: {}, base_uri: {}, dependencies: {:?}, exposure: {}, endpoint: {} metrics: {} }}",
               self.svc_id, self.name, self.version, self.online, self.description, self.health_check_uri, self.base_uri, self.dependencies, self.exposure, self.endpoint, self.metrics
        )
    }
}
