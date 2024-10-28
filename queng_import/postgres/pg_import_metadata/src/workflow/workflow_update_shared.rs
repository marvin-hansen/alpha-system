use crate::print_utils;
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

pub(crate) async fn update_assets_metadata(
    _dbm_mddb: &PostgresMDDBManager,
    _meta_data: &MetaDataSet,
) {
    print_utils::dbg_print("update_assets_metadata");

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
