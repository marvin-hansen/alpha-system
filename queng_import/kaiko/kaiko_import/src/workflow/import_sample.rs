use crate::print_utils;
use common_metadata::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

/// Imports a sample of asset metadata into the database.
///
/// # Arguments
/// * `dbm_mddb` - `PostgreSQL` metadata database manager
/// * `meta_data` - Source metadata set containing assets
/// * `sample_size` - Number of assets to import (must be less than total assets)
///
/// # Returns
/// The number of assets in the database after import
///
pub async fn import_assets_metadata_sample(
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
    sample_size: usize,
) -> Result<usize, &'static str> {
    print_utils::dbg_print("import_assets_metadata_sample");

    assert!(sample_size <= meta_data.assets().data.len());

    print_utils::dbg_print("Importing assets");
    let expected_asset_count = sample_size;

    let mut counter = 0;
    for asset in &meta_data.assets().data {
        if counter < sample_size {
            dbm_mddb
                .insert_asset(asset.clone())
                .await
                .expect("Failed to import assets");
            counter += 1;
        }
    }

    let db_asset_count = dbm_mddb
        .count_assets()
        .await
        .expect("Failed to count assets") as usize;

    print_utils::dbg_print(&format!("db_asset_count: {db_asset_count}"));
    print_utils::dbg_print(&format!("expected_asset_count: {expected_asset_count}"));

    print_utils::dbg_print("Completed importing assets");

    Ok(db_asset_count)
}

/// Imports a sample of exchange metadata into the database.
///
/// # Arguments
///
/// * `dbm_mddb` - The metadata database manager.
/// * `meta_data` - The metadata set containing exchange data.
/// * `sample_size` - The number of exchanges to import.
///
/// # Returns
///
/// The total number of exchanges in the database after import.
///
pub async fn import_exchanges_metadata_sample(
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
    sample_size: usize,
) -> Result<usize, &'static str> {
    print_utils::dbg_print("import_exchange_metadata_sample");

    assert!(sample_size <= meta_data.exchanges().data.len());

    print_utils::dbg_print("Importing exchanges");
    let expected_exchange_count = sample_size;

    let mut counter = 0;
    for exchange in &meta_data.exchanges().data {
        if counter < sample_size {
            dbm_mddb
                .insert_exchange(exchange.clone())
                .await
                .expect("Failed to import exchanges");
            counter += 1;
        }
    }

    let db_exchange_count = dbm_mddb
        .count_exchanges()
        .await
        .expect("Failed to count exchanges") as usize;

    print_utils::dbg_print(&format!("db_exchange_count: {db_exchange_count}"));
    print_utils::dbg_print(&format!(
        "expected_exchange_count: {expected_exchange_count}"
    ));

    print_utils::dbg_print("Completed importing exchanges");

    Ok(db_exchange_count)
}

/// Imports a sample of instruments metadata into the database.
///
/// # Arguments
/// * `dbm_mddb` - `PostgreSQL` metadata database manager
/// * `meta_data` - Source metadata set containing instruments
/// * `sample_size` - Number of instruments to import (must be less than total instruments)
///
/// # Returns
/// Number of instruments in database after import
///
pub async fn import_instruments_metadata_sample(
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
    sample_size: usize,
) -> Result<usize, &'static str> {
    print_utils::dbg_print("import_instruments_metadata_sample");

    assert!(sample_size <= meta_data.instruments().data.len());

    print_utils::dbg_print("Importing instruments");
    let expected_instrument_count = sample_size;

    let instruments = meta_data.instruments().data.as_slice();
    let mut data = Vec::from(instruments);
    data.truncate(sample_size);
    data.shrink_to_fit();
    assert_eq!(data.len(), sample_size);

    // Batch insert is usually a lot faster. i.e. 10x faster than insert one by one.
    dbm_mddb
        .insert_instrument_collection(&data)
        .await
        .expect("Failed to import instruments");

    let db_instrument_count = dbm_mddb
        .count_instruments()
        .await
        .expect("Failed to count instruments") as usize;

    print_utils::dbg_print(&format!("db_instrument_count: {db_instrument_count}"));
    print_utils::dbg_print(&format!(
        "expected_instrument_count: {expected_instrument_count}"
    ));

    print_utils::dbg_print("Completed importing instruments");

    Ok(db_instrument_count)
}
