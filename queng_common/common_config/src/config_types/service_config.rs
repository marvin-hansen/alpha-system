use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::prelude::{Endpoint, MetricConfig, ProtocolType, ServiceID, ServiceType};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct ServiceConfig {
    /// Service ID.
    svc_id: ServiceID,
    /// Service name.
    name: String,
    /// Service version.
    version: u32,
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
    /// * `svc_id` - Service ID.
    /// * `name` - Service name.
    /// * `version` - Service version.
    /// * `online` - Whether the service is online.
    /// * `description` - Service description.
    /// * `health_check_uri` - Health check URI.
    /// * `base_uri` - Base URI.
    /// * `dependencies` - Service dependencies.
    /// * `exposure` - Service exposure type.
    /// * `endpoint` - Service endpoint.
    /// * `metrics` - MetricConfig.
    // https://rust-lang.github.io/rust-clippy/master/index.html#/too_many_arguments
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        svc_id: ServiceID,
        name: String,
        version: u32,
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
    /// Converts a SQL row into a ServiceConfig object.
    ///
    /// This function takes a sql `Row` object, which is typically returned by a SQL query,
    /// and converts it into a `ServiceConfig` object. The `Row` object must have
    /// the following fields in the specified order:
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
    /// * `metrics` - MetricConfig.
    pub fn from_sql_row(row: &Row) -> Self {
        let db_id = row.get::<usize, i32>(0);
        let db_name = row.get::<usize, String>(1);
        let db_version = row.get::<usize, i16>(2);
        let db_online = row.get::<usize, bool>(3);
        let db_description = row.get::<usize, String>(4);
        let db_health_check_uri = row.get::<usize, String>(5);
        let db_base_uri = row.get::<usize, String>(6);
        let db_dependencies = row.get::<usize, Vec<i16>>(7);
        let db_exposure = row.get::<usize, i16>(8);

        let db_endpoint_name = row.get::<usize, String>(9);
        let db_endpoint_version = row.get::<usize, i16>(10);
        let db_endpoint_uri = row.get::<usize, String>(11);
        let db_endpoint_port = row.get::<usize, i16>(12);
        let db_endpoint_protocol = row.get::<usize, i16>(13);

        let db_metrics_uri = row.get::<usize, String>(14);
        let db_metrics_host = row.get::<usize, String>(15);
        let db_metrics_port = row.get::<usize, i16>(16);

        let dependencies: Vec<ServiceID> = db_dependencies
            .iter()
            .map(|id| ServiceID::from(*id))
            .collect();

        ServiceConfig::new(
            ServiceID::from(db_id),
            db_name,
            db_version as u32,
            db_online,
            db_description,
            db_health_check_uri,
            db_base_uri,
            dependencies,
            ServiceType::from(db_exposure),
            Endpoint::new(
                db_endpoint_name,
                db_endpoint_version as u32,
                db_endpoint_uri,
                db_endpoint_port as u32,
                ProtocolType::from(db_endpoint_protocol),
            ),
            MetricConfig::new(db_metrics_uri, db_metrics_host, db_metrics_port as u32),
        )
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
    pub fn version(&self) -> u32 {
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
