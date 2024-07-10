use crate::db::specs::{Specs, DB_NAME};
use crate::prelude::ClickHouseUtilError;

impl Specs {
    pub(crate) async fn create_service_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.generate_create_service_table_ddl();
        match self.execute_query(&ddl).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClickHouseUtilError::from(e.to_string())),
        }
    }

    fn generate_create_service_table_ddl(&self) -> String {
        format!("CREATE TABLE IF NOT EXISTS {DB_NAME}.instruments")
    }
}
