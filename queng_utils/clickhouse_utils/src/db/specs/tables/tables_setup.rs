use crate::db::specs::{Specs, DB_NAME, DB_TABLES};
use crate::prelude::ClickHouseUtilError;

impl Specs {
    pub async fn create_all_specs_tables(&self) -> Result<(), ClickHouseUtilError> {
        self.dbg_print("/create_all_specs_tables: create_service_table");
        self.create_service_table()
            .await
            .expect("[ClickhouseUtil]/create_all_specs_tables: Failed to create service table");

        Ok(())
    }

    pub async fn verify_all_specs_tables(&self) -> Result<bool, ClickHouseUtilError> {
        self.verify_specs_tables_created().await
    }

    async fn verify_specs_tables_created(&self) -> Result<bool, ClickHouseUtilError> {
        let tables = DB_TABLES;
        for table_name in tables {
            let query = format!("EXISTS TABLE {DB_NAME}.{table_name};");
            match self.verify_table_exists(&query).await {
                Ok(exists) => {
                    if !exists {
                        return Ok(false);
                    }
                }
                Err(e) => return Err(ClickHouseUtilError::from(e.to_string())),
            }
        }

        Ok(true)
    }
}
