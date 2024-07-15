use crate::db::{Specs, SERVICE_TABLE};
use crate::prelude::SurrealUtilError;
use crate::query_utils::ddl_utils;

impl Specs {
    pub async fn create_service_table(&self) -> Result<(), SurrealUtilError> {
        let ddl = ddl_utils::generate_create_table_ddl(SERVICE_TABLE);
        match self.db.query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(crate::prelude::SurrealUtilError(e.to_string())),
        }
    }
}
