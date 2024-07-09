use crate::ClickhouseUtil;
use std::error::Error;

impl ClickhouseUtil {
    pub async fn setup_all_db(&self) -> Result<(), Box<dyn Error>> {
        self.dbg_print("[setup_all_db]: create_metadata_db");
        self.metadata
            .create_metadata_db()
            .await
            .expect("[setup_db]: Failed to create metadata DB");

        self.dbg_print("[setup_all_db]: create_spec_db");
        self.specs
            .create_spec_db()
            .await
            .expect("[setup_db]: Failed to create specs DB");

        Ok(())
    }

    pub async fn verify_all_db_exists(&self) -> Result<bool, Box<dyn Error>> {
        self.dbg_print("[setup_all_db]: verify_metadata_db_exists");
        let metadata_db = self
            .metadata
            .verify_metadata_db_exists()
            .await
            .expect("[setup_db]: Failed to verify if metadata DB exists");

        let specs_db = self
            .specs
            .verify_specs_db_exists()
            .await
            .expect("[setup_db]: Failed to verify if specs DB exists");

        let all_db = metadata_db && specs_db;

        return Ok(all_db);
    }

    pub async fn teardown_all_db(&self) -> Result<(), Box<dyn Error>> {
        self.dbg_print("[teardown_all_db]: drop_metadata_db");
        self.metadata
            .drop_metadata_db()
            .await
            .expect("[teardown_db]: Failed to drop metadata DB");

        self.dbg_print("[teardown_all_db]: drop_spec_db");
        self.specs
            .drop_spec_db()
            .await
            .expect("[teardown_db]: Failed to drop specs DB");

        Ok(())
    }
}
