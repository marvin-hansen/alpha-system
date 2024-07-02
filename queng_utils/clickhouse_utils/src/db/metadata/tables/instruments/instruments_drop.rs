use crate::db::metadata::{Metadata, DB_NAME};
use crate::types::error::ClickHouseUtilError;

impl Metadata {
    pub(crate) async fn drop_instruments_table(&self) -> Result<(), ClickHouseUtilError> {
        let ddl = self.generate_drop_instruments_table_ddl();
        self.execute_query(&ddl)
            .await
            .expect("Failed to drop instruments table");

        Ok(())
    }

    fn generate_drop_instruments_table_ddl(&self) -> String {
        format!("DROP TABLE IF EXISTS {DB_NAME}.instruments")
    }
}
