use crate::{MetadataImportManager, AUTO_DETECT_PROXY};
use kaiko_import::prelude::{determine_workflow, execute_workflow, MetaDataDBWOp, WorkflowOpAll};
use std::error::Error;

impl MetadataImportManager {
    pub async fn determine_workflow(
        &self,
        sample_size: Option<(usize, usize, usize)>,
    ) -> Result<MetaDataDBWOp, Box<dyn Error>> {
        self.dbg_print("determine_workflow");

        self.dbg_print("Download metadata stats from Kaiko / Proxy");
        let meta_data_stats = kaiko_download::download_meta_data_stats(self.dbg, AUTO_DETECT_PROXY)
            .await
            .expect("Failed to download metadata stats");

        self.dbg_print("Reading metadata records from Database");
        let meta_data_db = self
            .dbm
            .count_metadata_records()
            .await
            .expect("Failed to load metadata from DB");

        self.dbg_print("Determine workflow");
        let workflow = determine_workflow(&meta_data_stats, &meta_data_db, sample_size).await;

        Ok(workflow)
    }

    pub async fn execute_workflow(&self, workflow: &MetaDataDBWOp) -> Result<(), Box<dyn Error>> {
        if workflow.all_op() == WorkflowOpAll::NoOPAll {
            return Ok(());
        }

        self.dbg_print("Download metadata");
        let meta_data = kaiko_download::download_meta_data(self.dbg, AUTO_DETECT_PROXY)
            .await
            .expect("Failed to download metadata");

        self.dbg_print("Import metadata into Database");
        execute_workflow(&self.dbm, &meta_data, workflow).await;

        Ok(())
    }
}
