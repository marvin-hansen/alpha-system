use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn create_service_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_service_table");

        let types_ddl = self.generate_service_table_types_ddl();
        self.execute_query(&types_ddl)
            .await
            .expect("Failed to create composite types for service table");

        let ddl = self.generate_service_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
