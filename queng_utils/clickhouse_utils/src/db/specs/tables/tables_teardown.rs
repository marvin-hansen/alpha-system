use crate::db::specs::Specs;
use crate::prelude::ClickHouseUtilError;

impl Specs {
    pub async fn drop_all_specs_tables(&self) -> Result<(), ClickHouseUtilError> {
        self.dbg_print("/create_all_specs_tables: drop_services_table");
        self.drop_services_table()
            .await
            .expect("[ClickhouseUtil]/create_all_specs_tables: Failed to drop service table");

        Ok(())
    }
}
