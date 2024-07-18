use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn create_instrument_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_instrument_table");

        let ddl = self.generate_instrument_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
