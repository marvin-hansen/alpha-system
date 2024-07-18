use crate::db::metadata::Metadata;
use crate::prelude::ClickHouseUtilError;

impl Metadata {
    pub async fn setup_metadata_db(&self) -> Result<(), ClickHouseUtilError> {
        self.dbg_print("setup_metadata_db");
        match self.create_metadata_db().await {
            Ok(()) => (),
            Err(e) => return Err(e),
        }

        self.dbg_print("verify_metadata_db_exists");
        match self.verify_metadata_db_exists().await {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        self.dbg_print("create_all_metadata_tables");
        match self.create_all_metadata_tables().await {
            Ok(()) => (),
            Err(e) => return Err(e),
        };

        self.dbg_print("verify_all_metadata_tables");
        match self.verify_all_metadata_tables().await {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        Ok(())
    }
}
