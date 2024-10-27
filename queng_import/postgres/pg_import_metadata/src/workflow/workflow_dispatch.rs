use crate::workflow::worflow_op::{MetaDataDBWOp, WorkflowOpAll};
use crate::{print_utils, workflow};
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;
use std::process::exit;

pub(crate) async fn dispatch_workflow(
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
    meta_data_ops: MetaDataDBWOp,
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

            // let assets_op: WorkflowOp = meta_data_ops.assets_op();
            // let exchanges_op: WorkflowOp = meta_data_ops.exchanges_op();
            // let instruments_op: WorkflowOp = meta_data_ops.instruments_op();
        }

        WorkflowOpAll::UpdateAll => {
            print_utils::dbg_print("UpdateAll");
        }

        WorkflowOpAll::UpdatePartial => {
            // let assets_op: WorkflowOp = meta_data_ops.assets_op();
            // let exchanges_op: WorkflowOp = meta_data_ops.exchanges_op();
            // let instruments_op: WorkflowOp = meta_data_ops.instruments_op();

            print_utils::dbg_print("UpdateAssets");
        }
    }
}
