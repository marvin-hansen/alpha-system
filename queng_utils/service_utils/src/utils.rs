use common_config::prelude::ServiceID;

use crate::error::ServiceUtilError;
use crate::ServiceUtil;

impl ServiceUtil {
    pub(crate) fn get_service_target(&self, svc: &ServiceID) -> Result<String, ServiceUtilError> {
        match svc {
            ServiceID::CMDB => Ok(self.format_service_target(svc)),
            ServiceID::DBGW => Ok(self.format_service_target(svc)),
            ServiceID::QDGW => Ok(self.format_service_target(svc)),
            ServiceID::MDDB => Ok(self.format_service_target(svc)),
            ServiceID::VEX => Ok(self.format_service_target(svc)),
            ServiceID::KaikoProxy => Ok(self.format_aux_target(svc)),
            _ => Err(ServiceUtilError::ServiceNotSupported(format!(
                "Service {} not supported",
                svc
            ))),
        }
    }

    fn format_service_target(&self, svc: &ServiceID) -> String {
        format!(
            "./target-bzl/bin/queng_services/{}/bin",
            svc.to_string().to_lowercase()
        )
    }

    fn format_aux_target(&self, svc: &ServiceID) -> String {
        format!(
            "./target-bzl/bin/queng_services/aux/{}/bin",
            svc.to_string().to_lowercase()
        )
    }
}
