use crate::common::all_db_constants::{
    SERVICE_TABLE, SERVICE_TABLE_ENDPOINT_TYPE, SERVICE_TABLE_INDEX,
    SERVICE_TABLE_METRIC_CONFIG_TYPE,
};
use crate::common::common_ddl::{ddl_index, ddl_table, ddl_type};
use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Drops the service table and its associated types from the database.
    ///
    /// This method is responsible for dropping the service table and its associated types from the database.
    /// It performs the following steps:
    ///
    /// 1. Drops the `service_index` using the `ddl_index::generate_drop_index_ddl` function.
    /// 2. Drops the `service_table` using the `ddl_table::generate_drop_table_ddl` function.
    /// 3. Drops the `service_table_endpoint_type` using the `ddl_type::generate_drop_type_ddl` function.
    /// 4. Drops the `service_table_metric_config_type` using the `ddl_type::generate_drop_type_ddl` function.
    ///
    /// If the dropping operation is successful, it returns `Ok(())`. Otherwise, it returns an `Err` variant of `PostgresUtilError`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the service table and its associated types are dropped successfully.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error while dropping the service table and its associated types.
    ///
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
