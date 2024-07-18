use crate::db::all_db_constants::SERVICE_TABLE;
use crate::db::ddl::ddl_table;
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn drop_service_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_service_table");
        let ddl = &ddl_table::generate_drop_table_ddl(SERVICE_TABLE);
        match self.execute_query(ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
