use crate::db::{Specs, SERVICES_TABLE};
use crate::prelude::SurrealUtilError;
use crate::query_utils::ddl_utils;

impl Specs {
    pub async fn drop_service_table(&self) -> Result<(), SurrealUtilError> {
        let ddl = ddl_utils::generate_drop_table_ddl(SERVICES_TABLE);
        match self.db.query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SurrealUtilError::new(e.to_string())),
        }
    }
}
