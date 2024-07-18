use crate::db::metadata::Metadata;
use crate::error::ClickHouseUtilError;

impl Metadata {
    pub async fn teardown(&self, drop_db: bool) -> Result<(), ClickHouseUtilError> {
        self.dbg_print("[teardown]: drop_all_metadata_tables");
        match self.drop_all_metadata_tables().await {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        if drop_db {
            self.dbg_print("[teardown]: drop_metadata_db");
            match self.drop_metadata_db().await {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}
