use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    /// Creates the service table, including metric config, endpoint type, and indexes in the database.
    ///
    /// This method is responsible for creating the service table, metric config, endpoint type, and indexes in the database.
    /// It performs the following steps:
    ///
    ///
    /// 2. Generates and executes the DDL for creating the endpoint type table using `generate_service_table_endpoint_ddl`.
    /// 3. Generates and executes the DDL for creating the service table using `generate_service_table_ddl`.
    /// 4. Generates and executes the DDL for creating the service table indexes using `generate_service_table_index_ddl`.
    ///
    /// If the creation operation is successful, it returns `Ok(())`. Otherwise, it returns an `Err` variant of `PostgresUtilError`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the service table, metric config, endpoint type, and indexes are created successfully.
    ///
    /// # Errors
    ///
    /// Returns an `Err` variant of `PostgresUtilError` if there is an error
    /// during the creation of the service table, metric config, endpoint type, or indexes.
    ///
    /// # Remarks
    ///
    /// This method is asynchronous and should be awaited in an asynchronous context.
    /// It is important to handle errors properly when creating the service table,
    /// metric config, endpoint type, and indexes.
    ///
    /// # Safety
    ///
    /// This method assumes the correctness of the underlying table and index creation mechanism.
    /// Ensure that the table and index creation operations are intended
    /// and the implications of creating the service table, metric config, endpoint type, and indexes
    /// are understood before calling this method.
    ///
    /// # Panics
    ///
    /// This method does not panic under normal circumstances.
    /// Any unexpected behavior should result in an `Err` variant being returned.
    ///
    /// # Aborts
    ///
    /// This method does not abort the program. It provides a controlled way
    /// to create the service table, metric config, endpoint type, and indexes.
    ///
    pub async fn create_service_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_protocol_type");
        let ddl = self.generate_protocol_type_enum_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => (),
            Err(e) => return Err(PostgresUtilError::new(e.to_string())),
        };

        self.dbg_print("drop_endpoint_type");
        let ddl = self.generate_drop_endpoint_type_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => (),
            Err(e) => return Err(PostgresUtilError::new(e.to_string())),
        };

        self.dbg_print("create_endpoint_type");
        let type_ddl = self.generate_endpoint_type_ddl();
        match self.execute_query(&type_ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to create endpoint type for service table: {}",
                    e
                )))
            }
        };

        self.dbg_print("create_service_table");
        let table_ddl = self.generate_service_table_ddl();
        match self.execute_query(&table_ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to create service table: {}",
                    e
                )))
            }
        };

        self.dbg_print("create_service_table/index");
        let indexes_ddl = self.generate_service_table_index_ddl();
        match self.execute_query(&indexes_ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to create service table indexes: {}",
                    e
                )))
            }
        };

        Ok(())
    }
}
