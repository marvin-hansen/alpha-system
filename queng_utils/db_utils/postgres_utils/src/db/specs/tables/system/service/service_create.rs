use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn create_service_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_service_table");

        self.dbg_print("create_service_table/metric_config type");
        let metric_config_ddl = self.generate_service_table_metric_config_ddl();
        match self.execute_query(&metric_config_ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to create service table metric config: {}",
                    e
                )))
            }
        }

        self.dbg_print("create_service_table/endpoint type");
        let endpoint_ddl = self.generate_service_table_endpoint_ddl();
        match self.execute_query(&endpoint_ddl).await {
            Ok(_) => (),
            Err(e) => {
                return Err(PostgresUtilError::new(format!(
                    "Failed to create service table endpoint type: {}",
                    e
                )))
            }
        }

        self.dbg_print("create_service_table");
        let ddl = self.generate_service_table_ddl();
        match self.execute_query(&ddl).await {
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
