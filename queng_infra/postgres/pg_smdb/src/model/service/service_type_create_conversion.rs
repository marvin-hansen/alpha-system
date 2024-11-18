use crate::model::endpoint_type::Endpoint;
use crate::model::service::CreateService;
use common_config::ServiceConfig as CommonServiceConfig;

impl CreateService {
    pub fn from_common_svc_config(common_svc_config: &CommonServiceConfig) -> CreateService {
        CreateService {
            service_id: common_svc_config.svc_id().as_i32(),
            name: common_svc_config.name().to_string(),
            version: common_svc_config.version() as i32,
            online: common_svc_config.online(),
            description: common_svc_config.description().to_string(),
            health_check_uri: common_svc_config.health_check_uri().to_string(),
            base_uri: common_svc_config.cluster_uri().to_string(),
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
}
