use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn drop_service_table(&self) -> Result<(), PostgresUtilError> {
        return Err(PostgresUtilError::from("Not implemented".to_string()));
    }
}
