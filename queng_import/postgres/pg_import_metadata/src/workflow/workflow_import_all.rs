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

    workflow::import_assets_metadata(dbm_mddb, meta_data).await;

    workflow::import_exchanges_metadata(dbm_mddb, meta_data).await;

    workflow::import_instruments_metadata(dbm_mddb, meta_data).await;
}
