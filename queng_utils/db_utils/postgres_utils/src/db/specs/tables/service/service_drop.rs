use crate::db::{Specs, SERVICE_TABLE};
use crate::prelude::PostgresUtilError;
use crate::query_utils::ddl_utils;

impl Specs {
    pub async fn drop_service_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_service_table");
        let ddl = &ddl_utils::generate_drop_table_ddl(SERVICE_TABLE);
        match self.execute_query(ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
