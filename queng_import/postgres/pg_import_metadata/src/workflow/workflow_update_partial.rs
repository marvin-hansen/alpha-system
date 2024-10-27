use crate::print_utils;
use crate::workflow;
use crate::workflow::worflow_op::{MetaDataDBWOp, WorkflowOp};
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

/// Update partial metadata in the database.
///
/// This function will update only the specified metadata in the database
/// using the given `PostgresMDDBManager`.
///
/// The `meta_data_ops` parameter specifies which assets, exchanges and
/// instruments should be updated.
///
/// `dbm_mddb` is the `PostgresMDDBManager` to use for creating the database
/// schema.
///
/// `meta_data` is the `MetaDataSet` containing the metadata to be imported.
///
pub(crate) async fn update_partial_metadata(
    meta_data_ops: &MetaDataDBWOp,
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
) -> () {
    print_utils::dbg_print("update_partial_metadata");

    let stats = meta_data.stats();

    if meta_data_ops.assets_op() == WorkflowOp::UpdateAssets {
        print_utils::dbg_print("Updating assets");
        let expected_asset_count = stats.number_assets() as usize;

        workflow::update_assets_metadata(dbm_mddb, meta_data).await;

        let db_asset_count = dbm_mddb
            .count_assets()
            .await
            .expect("Failed to count assets") as usize;

        assert_eq!(db_asset_count, expected_asset_count);

        print_utils::dbg_print("Completed updating assets");
    }

    if meta_data_ops.exchanges_op() == WorkflowOp::UpdateExchanges {
        print_utils::dbg_print("Updating exchanges");
        let expected_exchange_count = stats.number_exchanges() as usize;

        workflow::update_exchange_metadata(dbm_mddb, meta_data).await;

        let db_exchange_count = dbm_mddb
            .count_exchanges()
            .await
            .expect("Failed to count exchanges") as usize;

        assert_eq!(db_exchange_count, expected_exchange_count);

        print_utils::dbg_print("Completed updating exchanges");
    }

    if meta_data_ops.instruments_op() == WorkflowOp::UpdateInstruments {
        print_utils::dbg_print("Updating instruments");
        let expected_instrument_count = stats.number_instruments() as usize;

        workflow::update_instrument_metadata(dbm_mddb, meta_data).await;

        let db_instrument_count = dbm_mddb
            .count_instruments()
            .await
            .expect("Failed to count instruments") as usize;

        assert_eq!(db_instrument_count, expected_instrument_count);

        print_utils::dbg_print("Completed updating instruments");
    }
}
