/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::print_utils;
use crate::workflow;
use common_metadata::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

/// Update all metadata in the database.
///
/// This function will update all assets, exchanges and instruments in the database
/// using the given `PostgresMDDBManager`.
///
/// `dbm_mddb` is the `PostgresMDDBManager` to use for creating the database
/// schema.
///
/// `meta_data` is the `MetaDataSet` containing the metadata to be updated.
///
pub async fn update_all_metadata(dbm_mddb: &PostgresMDDBManager, meta_data: &MetaDataSet) {
    print_utils::dbg_print("update_all_metadata");

    workflow::update_assets_metadata(dbm_mddb, meta_data).await;

    workflow::update_exchanges_metadata(dbm_mddb, meta_data).await;

    workflow::update_instruments_metadata(dbm_mddb, meta_data).await;
}
