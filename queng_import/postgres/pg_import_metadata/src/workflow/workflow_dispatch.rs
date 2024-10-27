use crate::workflow::worflow_op::{MetaDataDBWOp, WorkflowOpAll};
use crate::{print_utils, workflow};
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;
use std::process::exit;

pub(crate) async fn dispatch_workflow(
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
    meta_data_ops: &MetaDataDBWOp,
) -> () {
    print_utils::dbg_print("dispatch_workflow");

    match meta_data_ops.all_op() {
        WorkflowOpAll::NoOPAll => {
            print_utils::dbg_print("NoOP");
            print_utils::print_already_imported_header();
            exit(0);
        }
        WorkflowOpAll::ImportAll => {
            print_utils::dbg_print("Importing All Metadata");
            workflow::import_all_metadata(dbm_mddb, meta_data).await;
        }

        WorkflowOpAll::ImportPartial => {
            print_utils::dbg_print("PartialUpdate");
            workflow::import_partial_metadata(meta_data_ops, dbm_mddb, meta_data).await;
        }

        WorkflowOpAll::UpdateAll => {
            print_utils::dbg_print("UpdateAll");
        }

        WorkflowOpAll::UpdatePartial => {
            print_utils::dbg_print("UpdateAssets");
        }
    }
}
