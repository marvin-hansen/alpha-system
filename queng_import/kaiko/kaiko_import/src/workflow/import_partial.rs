use crate::types::worflow_op::{MetaDataDBWOp, WorkflowOp};
use crate::{print_utils, workflow};
use common_metadata::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

/// Import partial metadata into the database.
///
/// This function will import only the specified metadata into the database
/// using the given `PostgresMDDBManager`.
///
pub async fn import_partial_metadata(
    meta_data_ops: &MetaDataDBWOp,
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
) {
    print_utils::dbg_print("import_partial_metadata");
    if meta_data_ops.assets_op() == WorkflowOp::ImportAssets {
        print_utils::dbg_print("ImportAssets");
        let imported_asset_count = workflow::import_assets_metadata(dbm_mddb, meta_data)
            .await
            .expect("Failed to import assets");
        print_utils::print_import_header("Imported Assets", imported_asset_count);
    }

    if meta_data_ops.exchanges_op() == WorkflowOp::ImportExchanges {
        print_utils::dbg_print("ImportExchanges");
        let imported_exchange_count = workflow::import_exchanges_metadata(dbm_mddb, meta_data)
            .await
            .expect("Failed to import exchanges");
        print_utils::print_import_header("Imported Exchanges", imported_exchange_count);
    }

    if meta_data_ops.instruments_op() == WorkflowOp::ImportInstruments {
        print_utils::dbg_print("ImportInstruments");
        let imported_instrument_count = workflow::import_instruments_metadata(dbm_mddb, meta_data)
            .await
            .expect("Failed to import instruments");
        print_utils::print_import_header("Imported Instruments", imported_instrument_count);
    }
}
