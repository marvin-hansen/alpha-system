use crate::print_utils;
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

/// Asynchronously imports assets metadata into the Postgres database.
///
/// # Arguments
///
/// * `dbm_mddb` - A reference to the PostgresMDDBManager for database operations.
/// * `meta_data` - A reference to the MetaDataSet containing assets metadata.
///
/// # Panics
///
/// Panics if there is a failure during the import process.
///
/// # Assertions
///
/// Asserts that the number of assets imported matches the expected asset count.
///
/// # Returns
///
/// This function does not return a value but prints debug messages during the import process.
///
pub(crate) async fn import_assets_metadata(
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
) {
    print_utils::dbg_print("import_assets_metadata");
    let stats = meta_data.stats();

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

/// Asynchronously imports exchange metadata into the PostgresMDDBManager.
///
/// # Arguments
///
/// * `dbm_mddb` - A reference to the PostgresMDDBManager for database operations.
/// * `meta_data` - A reference to the MetaDataSet containing exchange metadata.
///
/// # Panics
///
/// Panics if any database operation fails during the import process.
///
/// # Assertions
///
/// Asserts that the number of exchanges imported matches the expected count.
///
/// # Returns
///
/// This function does not return a value but prints debug messages during the import process.
///
pub(crate) async fn import_exchanges_metadata(
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
) {
    print_utils::dbg_print("import_exchange_metadata");
    let stats = meta_data.stats();

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

/// Asynchronously imports instruments metadata into the Postgres metadata database.
///
/// # Arguments
///
/// - `dbm_mddb`: A reference to the PostgresMDDBManager for interacting with the database.
/// - `meta_data`: A reference to the MetaDataSet containing the instruments metadata to import.
///
/// # Panics
///
/// Panics if there is a failure during the import process or
/// if the number of imported instruments does not match the expected count.
///
/// # Assertions
///
/// Asserts that the number of imported instruments matches the expected count.
///
/// # Returns
///
/// This function does not return a value but prints debug messages during the import process.
///
pub(crate) async fn import_instruments_metadata(
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
) {
    print_utils::dbg_print("import_instruments_metadata");
    let stats = meta_data.stats();

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
