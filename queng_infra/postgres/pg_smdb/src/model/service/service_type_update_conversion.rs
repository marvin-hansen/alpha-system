use crate::model::endpoint_type::Endpoint;
use crate::model::service::UpdateService;
use common_config::prelude::ServiceConfig as CommonServiceConfig;

impl UpdateService {
    pub fn from_common_svc_config(common_svc_config: &CommonServiceConfig) -> UpdateService {
        UpdateService {
            name: Some(common_svc_config.name().to_string()),
            version: Some(common_svc_config.version() as i32),
            online: Some(common_svc_config.online()),
            description: Some(common_svc_config.description().to_string()),
            health_check_uri: Some(common_svc_config.health_check_uri().to_string()),
            base_uri: Some(common_svc_config.cluster_uri().to_string()),
            dependencies: Some(
                common_svc_config
                    .dependencies()
                    .iter()
                    .map(|id| Some(id.as_i32()))
                    .collect(),
            ),
            endpoints: Some(
                common_svc_config
                    .endpoints()
                    .iter()
                    .map(|endpoint| Some(Endpoint::from_common_endpoint(endpoint)))
                    .collect(),
            ),
        }
    }
}
