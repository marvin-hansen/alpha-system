use crate::workflow::worflow_op::{MetaDataDBWOp, WorkflowOp};
use crate::{print_utils, workflow};
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

/// Import partial metadata into the database.
///
/// This function will import only the specified metadata into the database
/// using the given `PostgresMDDBManager`.
///
pub(crate) async fn import_partial_metadata(
    meta_data_ops: &MetaDataDBWOp,
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
) {
    print_utils::dbg_print("import_partial_metadata");

    if meta_data_ops.assets_op() == WorkflowOp::ImportAssets {
        workflow::import_assets_metadata(dbm_mddb, meta_data).await;
    }

    if meta_data_ops.exchanges_op() == WorkflowOp::ImportExchanges {
        workflow::import_exchanges_metadata(dbm_mddb, meta_data).await;
    }

    if meta_data_ops.instruments_op() == WorkflowOp::ImportInstruments {
        workflow::import_instruments_metadata(dbm_mddb, meta_data).await;
    }
}
