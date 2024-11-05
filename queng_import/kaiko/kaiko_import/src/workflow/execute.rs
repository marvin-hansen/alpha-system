use crate::types::worflow_op::{MetaDataDBWOp, WorkflowOpAll};
use crate::{print_utils, workflow};
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

///
/// Dispatches the appropriate workflow based on the provided metadata operations.
///
/// This function determines which workflow operation to execute (e.g., no operation,
/// import all, import partial, update all, update partial) by matching against the
/// `meta_data_ops` parameter. It uses the provided `PostgresMDDBManager` to perform
/// database operations and the `MetaDataSet` to access the metadata.
///
/// # Parameters
///
/// - `dbm_mddb`: The `PostgresMDDBManager` to use for database operations.
/// - `meta_data`: The `MetaDataSet` containing the metadata to be processed.
/// - `meta_data_ops`: Specifies the workflow operations to be performed.
///
/// # Remarks
///
/// - This function logs the workflow operation being dispatched using `print_utils`.
/// - It delegates the actual workflow execution to specific functions from the
///   `workflow` module based on the operation type.
///
pub async fn execute_workflow(
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
    meta_data_ops: &MetaDataDBWOp,
) {
    print_utils::dbg_print("dispatch_workflow");

    match meta_data_ops.all_op() {
        WorkflowOpAll::NoOPAll => {
            print_utils::dbg_print("NoOP");
            print_utils::print_already_imported_header();
        }
        WorkflowOpAll::ImportAll => {
            print_utils::dbg_print("Import All Metadata");
            workflow::import_all_metadata(dbm_mddb, meta_data).await;
        }

        WorkflowOpAll::ImportSample(
            assets_sample_size,
            exchanges_sample_size,
            instruments_sample_size,
        ) => {
            print_utils::dbg_print("Import Sample Metadata");
            workflow::import_metadata_sample(
                dbm_mddb,
                meta_data,
                assets_sample_size,
                exchanges_sample_size,
                instruments_sample_size,
            )
            .await;
        }

        WorkflowOpAll::ImportPartial => {
            print_utils::dbg_print("Import Partial Metadata");
            workflow::import_partial_metadata(meta_data_ops, dbm_mddb, meta_data).await;
        }

        WorkflowOpAll::UpdateAll => {
            print_utils::dbg_print("Update All Metadata");
            workflow::update_all_metadata(dbm_mddb, meta_data).await;
        }

        WorkflowOpAll::UpdatePartial => {
            print_utils::dbg_print("Update Partial Metadata");
            workflow::update_partial_metadata(meta_data_ops, dbm_mddb, meta_data).await;
        }
    }
}
