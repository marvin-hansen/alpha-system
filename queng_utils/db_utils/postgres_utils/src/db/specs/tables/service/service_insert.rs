use crate::db::Specs;
use crate::prelude::SurrealUtilError;
use common_config::prelude::ServiceConfig;

impl Specs {
    pub async fn insert_service(&self, _data: &ServiceConfig) -> Result<(), SurrealUtilError> {
        return Err(SurrealUtilError::from("Not implemented".to_string()));
    }
}
