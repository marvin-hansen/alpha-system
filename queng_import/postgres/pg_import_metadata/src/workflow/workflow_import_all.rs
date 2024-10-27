use crate::print_utils;
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

/// Import all metadata into the database.
///
/// This function will import all assets, exchanges and instruments from the
/// given `MetaDataSet` into the database using the given `PostgresMDDBManager`.
///
pub(crate) async fn import_all_metadata(dbm_mddb: &PostgresMDDBManager, meta_data: &MetaDataSet) {
    let stats = meta_data.stats();
    let expected_asset_count = stats.number_assets() as usize;
    let expected_exchange_count = stats.number_exchanges() as usize;
    let expected_instrument_count = stats.number_instruments() as usize;

    print_utils::dbg_print("Import assets");
    let assets = meta_data.assets().data.as_slice();
    dbm_mddb
        .insert_asset_collection(assets)
        .await
        .expect("Failed to import assets");
    print_utils::dbg_print("Completed importing assets");

    print_utils::dbg_print("Import exchanges");
    let exchanges = meta_data.exchanges().data.as_slice();
    dbm_mddb
        .insert_exchange_collection(exchanges)
        .await
        .expect("Failed to import exchanges");
    print_utils::dbg_print("Completed importing exchanges");

    print_utils::dbg_print("Import instruments");
    let instruments = meta_data.instruments().data.as_slice();
    dbm_mddb
        .insert_instrument_collection(instruments)
        .await
        .expect("Failed to import instruments");
    print_utils::dbg_print("Completed importing instruments");

    let db_asset_count = dbm_mddb
        .count_assets()
        .await
        .expect("Failed to count assets") as usize;

    let db_exchange_count = dbm_mddb
        .count_exchanges()
        .await
        .expect("Failed to count exchanges") as usize;

    let db_instrument_count = dbm_mddb
        .count_instruments()
        .await
        .expect("Failed to count instruments") as usize;

    // Verify that all metadata was imported correctly
    assert_eq!(db_asset_count, expected_asset_count);
    assert_eq!(db_exchange_count, expected_exchange_count);
    assert_eq!(db_instrument_count, expected_instrument_count);

    print_utils::print_stop_header(db_asset_count, db_exchange_count, db_instrument_count);
}
