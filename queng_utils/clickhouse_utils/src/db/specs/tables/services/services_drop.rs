use crate::db::specs::{Specs, SERVICES_TABLE};
use crate::prelude::ClickHouseUtilError;

impl Specs {
    pub(crate) async fn drop_services_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.generate_drop_table_ddl(SERVICES_TABLE);
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop services table");

        Ok(())
    }
}
