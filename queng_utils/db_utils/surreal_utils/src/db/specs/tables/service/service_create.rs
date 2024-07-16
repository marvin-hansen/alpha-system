use crate::db::Specs;
use crate::prelude::SurrealUtilError;

impl Specs {
    pub async fn create_service_table(&self) -> Result<(), SurrealUtilError> {
        return Err(SurrealUtilError::from("Not implemented".to_string()));
    }
}
