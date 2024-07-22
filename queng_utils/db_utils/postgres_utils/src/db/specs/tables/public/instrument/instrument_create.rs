use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn create_instrument_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_instrument_table");
        let ddl = self.generate_instrument_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to create instrument table: {}",
                    e
                )))
            }
        };

        self.dbg_print("create_instrument_table/index");
        let indexes_ddl = self.generate_instrument_table_indexes_ddl();
        match self.execute_query(&indexes_ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to create instrument table indexes: {}",
                    e
                )))
            }
        };

        Ok(())
    }
}
