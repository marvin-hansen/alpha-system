use crate::db::Specs;
use crate::prelude::PostgresUtilError;

impl Specs {
    pub async fn create_service_table(&self) -> Result<(), PostgresUtilError> {
        self.dbg_print("create_service_table");

        self.dbg_print("create_service_table/metric_config");
        let metric_config_ddl = self.generate_service_table_metric_config_ddl();
        self.execute_query(&metric_config_ddl)
            .await
            .expect("Failed to create composite types for service table");

        self.dbg_print("create_service_table/endpoint");
        let endpoint_ddl = self.generate_service_table_endpoint_ddl();
        self.execute_query(&endpoint_ddl)
            .await
            .expect("Failed to create composite types for service table");

        self.dbg_print("create_service_table");
        let ddl = self.generate_service_table_ddl();
        self.execute_query(&ddl)
            .await
            .expect("Failed to create service table");

        self.dbg_print("create_service_table/indexes");
        let indexes_ddl = self.generate_service_table_indexes_ddl();
        match self.execute_query(&indexes_ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(PostgresUtilError::new(e.to_string())),
        }
    }
}
