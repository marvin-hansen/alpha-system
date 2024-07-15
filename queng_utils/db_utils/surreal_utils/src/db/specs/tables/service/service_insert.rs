use crate::db::Specs;
use crate::prelude::SurrealUtilError;
use common_config::prelude::ServiceConfig;

impl Specs {
    pub(crate) async fn insert_service_vec(
        &self,
        services: &[ServiceConfig],
    ) -> Result<(), SurrealUtilError> {
        match self.db.insert_service_vec(&services.to_vec()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SurrealUtilError::from(e.to_string())),
        }
    }

    pub(crate) async fn insert_service(
        &self,
        service: &ServiceConfig,
    ) -> Result<(), SurrealUtilError> {
        match self.db.insert_service(service).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SurrealUtilError::from(e.to_string())),
        }
    }
}
