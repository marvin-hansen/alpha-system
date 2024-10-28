use crate::print_utils;
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

/// Updates assets metadata in the Postgres database based on a comparison with the provided MetaDataSet.
///
/// # Arguments
///
/// * `dbm_mddb` - A reference to the PostgresMDDBManager for database operations.
/// * `meta_data` - A reference to the MetaDataSet containing assets metadata to be updated.
///
/// This function compares the existing assets in the database with the provided metadata and updates accordingly.
/// If the number of assets in the database is greater than expected, extra assets are deleted.
/// If the number of assets in the database is less than expected, missing assets are inserted.
/// It then compared all hashes of the existing assets with the hashes in the metadata and updates accordingly.
///
pub(crate) async fn update_assets_metadata(
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
) {
    print_utils::dbg_print("update_assets_metadata");

    let stats = meta_data.stats();
    let expected_asset_count = stats.number_assets() as usize;

    let db_assets = dbm_mddb
        .read_all_assets()
        .await
        .expect("Failed to load assets");
    let db_asset_count = db_assets.len();

    if db_asset_count > expected_asset_count {
        for db_asset in &db_assets {
            if !meta_data
                .assets()
                .data
                .iter()
                .any(|ma| ma.code == db_asset.code)
            {
                dbm_mddb
                    .delete_asset(db_asset.code.clone())
                    .await
                    .expect("Failed to delete extra asset");
            }
        }
    } else if db_asset_count < expected_asset_count {
        for meta_asset in meta_data.assets().data.iter() {
            if !db_assets.iter().any(|da| da.code == meta_asset.code) {
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
        if let Some(meta_asset) = meta_data
            .assets()
            .data
            .iter()
            .find(|ma| ma.code == db_asset.code)
        {
            if db_asset.hash() != meta_asset.hash() {
                dbm_mddb
                    .update_asset(db_asset.code.clone(), meta_asset.clone())
                    .await
                    .expect("Failed to update asset");
            }
        }
    }

    print_utils::dbg_print("Completed updating assets");
}

pub(crate) async fn update_exchanges_metadata(
    _dbm_mddb: &PostgresMDDBManager,
    _meta_data: &MetaDataSet,
) {
    print_utils::dbg_print("update_exchanges_metadata");

    print_utils::dbg_print("Completed updating exchanges");
}

pub(crate) async fn update_instruments_metadata(
    _dbm_mddb: &PostgresMDDBManager,
    _meta_data: &MetaDataSet,
) {
    print_utils::dbg_print("update_instruments_metadata");

    print_utils::dbg_print("Completed updating instruments");
}
