use crate::db::Specs;
use crate::prelude::SurrealUtilError;
use common_config::prelude::ServiceConfig;

impl Specs {
    pub(crate) async fn import_service_specs(
        &self,
        services: &[ServiceConfig],
    ) -> Result<(), SurrealUtilError> {
        for service in services.to_vec() {
            println!("{}", service)

            //self.db.insert_service(service).await.expect()
        }

        Ok(())
    }

    pub(crate) async fn insert_service(
        &self,
        service: &ServiceConfig,
    ) -> Result<(), SurrealUtilError> {
        let _ = self.db.insert_service(service.to_owned()).await.expect("");
        Ok(())
    }
}
