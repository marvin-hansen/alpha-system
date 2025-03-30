/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::print_utils;
use common_metadata::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

/// Updates assets metadata in the Postgres database based on a comparison with the provided `MetaDataSet`.
///
/// # Arguments
///
/// * `dbm_mddb` - A reference to the `PostgresMDDBManager` for database operations.
/// * `meta_data` - A reference to the `MetaDataSet` containing assets metadata to be updated.
///
/// This function compares the existing assets in the database with the provided metadata and updates accordingly.
/// If the number of assets in the database is greater than expected, extra assets are deleted.
/// If the number of assets in the database is less than expected, missing assets are inserted.
/// It then compared all hashes of the existing assets with the hashes in the metadata and updates accordingly.
///
pub async fn update_assets_metadata(dbm_mddb: &PostgresMDDBManager, meta_data: &MetaDataSet) {
    print_utils::dbg_print("update_assets_metadata");

    let stats = meta_data.stats();
    let expected_asset_count = stats.number_assets() as usize;

    let db_assets = dbm_mddb
        .read_all_assets()
        .await
        .expect("Failed to load assets");

    // find the assets that are in the database but not in the meta_data parameter,
    // and delete them from the database.
    for db_asset in &db_assets {
        if !meta_data
            .assets()
            .data
            .iter()
            .any(|ma| ma.code == db_asset.code)
        {
            // Double check that the ID really exists in the database
            let exists = dbm_mddb
                .check_if_asset_id_exists(db_asset.code.clone())
                .await
                .expect("Failed to check if asset exists");

            if exists {
                dbm_mddb
                    .delete_asset(db_asset.code.clone())
                    .await
                    .expect("Failed to delete extra asset");
            }
        }
    }

    // find the assets that are in the meta_data parameter but not in the database,
    // and insert them into the database.
    for meta_asset in &meta_data.assets().data {
        if !db_assets.iter().any(|da| da.code == meta_asset.code) {
            // Double check that the ID really does not exists already in the database
            //  This prevents unique key constraint violations
            let exists = dbm_mddb
                .check_if_asset_id_exists(meta_asset.code.clone())
                .await
                .expect("Failed to check if asset exists");
            if !exists {
                dbm_mddb
                    .insert_asset(meta_asset.clone())
                    .await
                    .expect("Failed to insert missing asset");
            }
        }
    }

    let db_assets = dbm_mddb
        .read_all_assets()
        .await
        .expect("Failed to reload assets");

    assert_eq!(
        db_assets.len(),
        expected_asset_count,
        "Asset count mismatch after update"
    );

    for db_asset in db_assets {
        //  match each asset code with the asset code in the meta_data parameter,
        if let Some(meta_asset) = meta_data
            .assets()
            .data
            .iter()
            .find(|ma| ma.code == db_asset.code)
        {
            if db_asset.hash() != meta_asset.hash() {
                // compare the asset hash of the database asset to the asset hash in the meta_data parameter,
                // and if the hashes don't match,
                // update the database asset in the database with the asset from the meta_data.
                dbm_mddb
                    .update_asset(db_asset.code.clone(), meta_asset.clone())
                    .await
                    .expect("Failed to update asset");
            }
        }
    }

    print_utils::dbg_print("Completed updating assets");
}

/// Asynchronously updates exchanges metadata based on a comparison between the database and the provided metadata.
///
/// # Arguments
///
/// - `dbm_mddb`: A reference to the `PostgresMDDBManager` for database operations.
/// - `meta_data`: A reference to the `MetaDataSet` containing exchange metadata.
///
/// This function compares the exchanges in the database with the ones in the `meta_data` parameter.
/// It deletes exchanges that are in the database but not in `meta_data` and inserts exchanges
/// that are in `meta_data` but not in the database.
/// Finally, it updates exchanges with different hashes.
///
/// Prints debug messages during the update process.
///
/// # Panics
///
/// Panics if any database operation fails during the update process.
///
/// # Assertions
///
/// Asserts that the number of exchanges in the database matches the expected count after the update.
///
pub async fn update_exchanges_metadata(dbm_mddb: &PostgresMDDBManager, meta_data: &MetaDataSet) {
    print_utils::dbg_print("update_exchanges_metadata");

    let stats = meta_data.stats();
    let expected_exchange_count = stats.number_exchanges() as usize;

    let db_exchanges = dbm_mddb
        .read_all_exchanges()
        .await
        .expect("Failed to load exchanges");

    // Find and delete exchanges that are in the database but not in the meta_data parameter
    for db_exchange in &db_exchanges {
        if !meta_data
            .exchanges()
            .data
            .iter()
            .any(|me| me.code == db_exchange.code)
        {
            let exists = dbm_mddb
                .check_if_exchange_id_exists(db_exchange.code.clone())
                .await
                .expect("Failed to check if exchange exists");

            if exists {
                dbm_mddb
                    .delete_exchange(db_exchange.code.clone())
                    .await
                    .expect("Failed to delete extra exchange");
            }
        }
    }

    // Find and insert exchanges that are in the meta_data parameter but not in the database
    for meta_exchange in &meta_data.exchanges().data {
        if !db_exchanges.iter().any(|de| de.code == meta_exchange.code) {
            let exists = dbm_mddb
                .check_if_exchange_id_exists(meta_exchange.code.clone())
                .await
                .expect("Failed to check if exchange exists");

            if !exists {
                dbm_mddb
                    .insert_exchange(meta_exchange.clone())
                    .await
                    .expect("Failed to insert missing exchange");
            }
        }
    }

    let db_exchanges = dbm_mddb
        .read_all_exchanges()
        .await
        .expect("Failed to reload exchanges");

    assert_eq!(
        db_exchanges.len(),
        expected_exchange_count,
        "Exchange count mismatch after update"
    );

    for db_exchange in db_exchanges {
        if let Some(meta_exchange) = meta_data
            .exchanges()
            .data
            .iter()
            .find(|me| me.code == db_exchange.code)
        {
            if db_exchange.hash() != meta_exchange.hash() {
                dbm_mddb
                    .update_exchange(db_exchange.code.clone(), meta_exchange.clone())
                    .await
                    .expect("Failed to update exchange");
            }
        }
    }

    print_utils::dbg_print("Completed updating exchanges");
}

/// Asynchronously updates instruments metadata based on a comparison between the database and the provided metadata.
///
/// # Arguments
///
/// - `dbm_mddb`: A reference to the `PostgresMDDBManager` for database operations.
/// - `meta_data`: A reference to the `MetaDataSet` containing instrument metadata.
///
/// This function compares the instruments in the database with the ones in the `meta_data` parameter.
/// It deletes instruments that are in the database but not in `meta_data` and inserts instruments
/// that are in `meta_data` but not in the database.
/// Finally, it updates instruments with different hashes.
///
/// Prints debug messages during the update process.
///
/// # Panics
///
/// Panics if any database operation fails during the update process.
///
/// # Assertions
///
/// Asserts that the number of instruments in the database matches the expected count after the update.
///
pub async fn update_instruments_metadata(dbm_mddb: &PostgresMDDBManager, meta_data: &MetaDataSet) {
    print_utils::dbg_print("update_instruments_metadata");

    let stats = meta_data.stats();
    let expected_instrument_count = stats.number_instruments() as usize;

    let db_instruments = dbm_mddb
        .read_all_instruments()
        .await
        .expect("Failed to load instruments");

    // Find and delete instruments that are in the database but not in the meta_data parameter
    for db_instrument in &db_instruments {
        if !meta_data
            .instruments()
            .data
            .iter()
            .any(|mi| mi.code == db_instrument.code)
        {
            let exists = dbm_mddb
                .check_if_instrument_id_exists(&db_instrument.primary_key())
                .await
                .expect("Failed to check if instrument exists");

            if exists {
                dbm_mddb
                    .delete_instrument(db_instrument.code.clone())
                    .await
                    .expect("Failed to delete extra instrument");
            }
        }
    }

    // Find and insert instruments that are in the meta_data parameter but not in the database
    for meta_instrument in &meta_data.instruments().data {
        if !db_instruments
            .iter()
            .any(|di| di.code == meta_instrument.code)
        {
            let exists = dbm_mddb
                .check_if_instrument_id_exists(&meta_instrument.primary_key())
                .await
                .expect("Failed to check if instrument exists");

            if !exists {
                dbm_mddb
                    .insert_instrument(meta_instrument.clone())
                    .await
                    .expect("Failed to insert missing instrument");
            }
        }
    }

    let db_instruments = dbm_mddb
        .read_all_instruments()
        .await
        .expect("Failed to reload instruments");

    assert_eq!(
        db_instruments.len(),
        expected_instrument_count,
        "Instrument count mismatch after update"
    );

    for db_instrument in db_instruments {
        if let Some(meta_instrument) = meta_data
            .instruments()
            .data
            .iter()
            .find(|mi| mi.code == db_instrument.code)
        {
            if db_instrument.hash() != meta_instrument.hash() {
                dbm_mddb
                    .update_instrument(&db_instrument.primary_key(), meta_instrument.clone())
                    .await
                    .expect("Failed to update instrument");
            }
        }
    }

    print_utils::dbg_print("Completed updating instruments");
}
