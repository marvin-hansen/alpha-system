use crate::model::service::Service;

use crate::model::endpoint_type::Endpoint;
use common_config::prelude::{ServiceConfig as CommonServiceConfig, ServiceID};

impl Service {
    pub fn from_common_svc_config(common_svc_config: &CommonServiceConfig) -> Service {
        Service {
            service_id: common_svc_config.svc_id().as_i32(),
            name: common_svc_config.name().to_string(),
            version: common_svc_config.version() as i32,
            online: common_svc_config.online(),
            description: common_svc_config.description().to_string(),
            health_check_uri: common_svc_config.health_check_uri().to_string(),
            base_uri: common_svc_config.base_uri().to_string(),
            dependencies: common_svc_config
                .dependencies()
                .iter()
                .map(|id| Some(id.as_i32()))
                .collect(),
            endpoints: common_svc_config
                .endpoints()
                .iter()
                .map(|endpoint| Some(Endpoint::from_common_endpoint(endpoint)))
                .collect(),
        }
    }

    pub fn to_common_svc_config(&self) -> CommonServiceConfig {
        CommonServiceConfig::new(
            ServiceID::from(self.service_id),
            self.name.clone(),
            self.version as u32,
            self.online,
            self.description.clone(),
            self.health_check_uri.clone(),
            self.base_uri.clone(),
            self.dependencies
                .iter()
                .flatten()
                .map(|id| ServiceID::from(*id))
                .collect(),
            self.endpoints
                .iter()
                .flatten()
                .map(|endpoint| endpoint.to_common_endpoint())
                .collect(),
        )
    }
}
