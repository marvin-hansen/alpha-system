use crate::error::ClickHouseUtilError;
use crate::ClickhouseUtil;

impl ClickhouseUtil {
    pub async fn verify_all_db_exists(&self) -> Result<bool, ClickHouseUtilError> {
        self.dbg_print("[verify_all_db]: verify_all_db_exists");
        let metadata_db_exists = self
            .metadata
            .verify_metadata_db_exists()
            .await
            .expect("[verify_db]: Failed to verify if metadata DB exists");

        let metadata_tables_exists = self
            .metadata
            .verify_all_metadata_tables()
            .await
            .expect("[verify_db]: Failed to verify if all metadata tables exists");

        let all_verify = metadata_db_exists && metadata_tables_exists;

        Ok(all_verify)
    }
}
