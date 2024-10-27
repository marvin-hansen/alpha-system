use crate::print_utils;
use crate::workflow::worflow_op::{MetaDataDBWOp, WorkflowOp};
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
) -> () {
    let stats = meta_data.stats();

    if meta_data_ops.assets_op() == WorkflowOp::ImportAssets {
        print_utils::dbg_print("Importing assets");
        let expected_asset_count = stats.number_assets() as usize;

        let assets = meta_data.assets().data.as_slice();
        dbm_mddb
            .insert_asset_collection(assets)
            .await
            .expect("Failed to import assets");

        let db_asset_count = dbm_mddb
            .count_assets()
            .await
            .expect("Failed to count assets") as usize;

        assert_eq!(db_asset_count, expected_asset_count);

        print_utils::dbg_print("Completed importing assets");
    }

    if meta_data_ops.exchanges_op() == WorkflowOp::ImportExchanges {
        print_utils::dbg_print("Importing exchanges");
        let expected_exchange_count = stats.number_exchanges() as usize;

        let exchanges = meta_data.exchanges().data.as_slice();
        dbm_mddb
            .insert_exchange_collection(exchanges)
            .await
            .expect("Failed to import exchanges");

        let db_exchange_count = dbm_mddb
            .count_exchanges()
            .await
            .expect("Failed to count exchanges") as usize;

        assert_eq!(db_exchange_count, expected_exchange_count);

        print_utils::dbg_print("Completed importing exchanges");
    }

    if meta_data_ops.instruments_op() == WorkflowOp::ImportInstruments {
        print_utils::dbg_print("Importing instruments");
        let expected_instrument_count = stats.number_instruments() as usize;

        let instruments = meta_data.instruments().data.as_slice();
        dbm_mddb
            .insert_instrument_collection(instruments)
            .await
            .expect("Failed to import instruments");

        let db_instrument_count = dbm_mddb
            .count_instruments()
            .await
            .expect("Failed to count instruments") as usize;

        assert_eq!(db_instrument_count, expected_instrument_count);

        print_utils::dbg_print("Completed importing instruments");
    }
}
