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
        let imported_asset_count = workflow::import_assets_metadata(dbm_mddb, meta_data).await;
        print_utils::print_import_header("Imported Assets", imported_asset_count);
    }

    if meta_data_ops.exchanges_op() == WorkflowOp::ImportExchanges {
        let imported_exchange_count =
            workflow::import_exchanges_metadata(dbm_mddb, meta_data).await;
        print_utils::print_import_header("Imported Exchanges", imported_exchange_count);
    }

    if meta_data_ops.instruments_op() == WorkflowOp::ImportInstruments {
        let imported_instrument_count =
            workflow::import_instruments_metadata(dbm_mddb, meta_data).await;
        print_utils::print_import_header("Imported Instruments", imported_instrument_count);
    }
}
