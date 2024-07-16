use crate::db::Specs;
use crate::prelude::PostgresUtilError;
use common_config::prelude::ServiceConfig;

impl Specs {
    pub async fn insert_service(&self, _data: &ServiceConfig) -> Result<(), PostgresUtilError> {
        return Err(PostgresUtilError::from("Not implemented".to_string()));
    }
}
