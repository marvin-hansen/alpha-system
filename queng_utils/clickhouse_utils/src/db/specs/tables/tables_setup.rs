use crate::db::specs::Specs;
use crate::prelude::ClickHouseUtilError;

impl Specs {
    pub async fn create_all_specs_tables(&self) -> Result<(), ClickHouseUtilError> {
        self.dbg_print("/create_all_specs_tables: create_service_table");
        self.create_service_table()
            .await
            .expect("[ClickhouseUtil]/create_all_specs_tables: Failed to create service table");

        Ok(())
    }
}
