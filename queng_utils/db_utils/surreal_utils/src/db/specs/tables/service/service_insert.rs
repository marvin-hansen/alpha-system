use crate::db::Specs;
use crate::prelude::SurrealUtilError;
use common_config::prelude::ServiceConfig;

impl Specs {
    pub async fn insert_service(&self, data: &ServiceConfig) -> Result<(), SurrealUtilError> {
        match self.db.insert_service(data).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SurrealUtilError::from(e.to_string())),
        }
    }
}
