use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn create_service_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_service_table");

        let ddl = "";
        match self.execute_query(ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
