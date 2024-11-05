use crate::{print_utils, workflow};
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

/// Import all metadata into the database.
///
/// This function will import all assets, exchanges and instruments from the
/// given `MetaDataSet` into the database using the given `PostgresMDDBManager`.
///
pub(crate) async fn import_all_metadata(dbm_mddb: &PostgresMDDBManager, meta_data: &MetaDataSet) {
    print_utils::dbg_print("import_all_metadata");

    // Import all metadata in parallel
    let (imported_asset_count, imported_exchange_count, imported_instrument_count) =
        tokio::try_join!(
            workflow::import_assets_metadata(dbm_mddb, meta_data),
            workflow::import_exchanges_metadata(dbm_mddb, meta_data),
            workflow::import_instruments_metadata(dbm_mddb, meta_data)
        )
        .expect("Failed to import metadata sample");

    print_utils::print_stop_header(
        imported_asset_count,
        imported_exchange_count,
        imported_instrument_count,
    )
}

/// Imports a sample of metadata into the database, including assets, exchanges, and instruments.
///
/// # Arguments
/// * `dbm_mddb` - PostgreSQL metadata database manager
/// * `meta_data` - Source metadata set to import from
/// * `sample_size` - Number of records to import for each category
///
/// # Returns
/// Nothing. Prints debug information about the import process and final counts.
///
pub(crate) async fn import_metadata_sample(
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
    assets_sample_size: usize,
    exchanges_sample_size: usize,
    instruments_sample_size: usize,
) {
    print_utils::dbg_print("import_metadata_sample");

    let imported_asset_count =
        workflow::import_assets_metadata_sample(dbm_mddb, meta_data, assets_sample_size)
            .await
            .expect("Failed to import assets");

    let imported_exchange_count =
        workflow::import_exchanges_metadata_sample(dbm_mddb, meta_data, exchanges_sample_size)
            .await
            .expect("Failed to import exchanges");

    let imported_instrument_count =
        workflow::import_instruments_metadata_sample(dbm_mddb, meta_data, instruments_sample_size)
            .await
            .expect("Failed to import instruments");

    print_utils::print_stop_header(
        imported_asset_count,
        imported_exchange_count,
        imported_instrument_count,
    );
}
