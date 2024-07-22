use crate::db::all_db_constants::{
    SERVICE_TABLE, SERVICE_TABLE_ENDPOINT_TYPE, SERVICE_TABLE_INDEX,
    SERVICE_TABLE_METRIC_CONFIG_TYPE,
};
use crate::db::common_ddl::{ddl_index, ddl_table, ddl_type};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn drop_service_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("drop_service_index");
        let ddl = &ddl_index::generate_drop_index_ddl(SERVICE_TABLE_INDEX);
        match self.execute_query(ddl).await {
            Ok(_) => (),
            Err(e) => return Err(PostgresUtilError::new(e.to_string())),
        };

        self.dbg_print("drop_service_table");
        let ddl = &ddl_table::generate_drop_table_ddl(SERVICE_TABLE);
        match self.execute_query(ddl).await {
            Ok(_) => (),
            Err(e) => return Err(PostgresUtilError::new(e.to_string())),
        };

        self.dbg_print("drop_service_table/endpoint_type");
        let ddl = &ddl_type::generate_drop_type_ddl(SERVICE_TABLE_ENDPOINT_TYPE);
        match self.execute_query(ddl).await {
            Ok(_) => (),
            Err(e) => return Err(PostgresUtilError::new(e.to_string())),
        };

        self.dbg_print("drop_service_table/metric_config_type");
        let ddl = &ddl_type::generate_drop_type_ddl(SERVICE_TABLE_METRIC_CONFIG_TYPE);
        match self.execute_query(ddl).await {
            Ok(_) => (),
            Err(e) => return Err(PostgresUtilError::new(e.to_string())),
        };

        Ok(())
    }
}
