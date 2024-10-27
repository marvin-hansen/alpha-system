use crate::print_utils;
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

pub(crate) async fn update_assets_metadata(
    _dbm_mddb: &PostgresMDDBManager,
    _meta_data: &MetaDataSet,
) -> () {
    print_utils::dbg_print("update_assets_metadata");
}

pub(crate) async fn update_exchange_metadata(
    _dbm_mddb: &PostgresMDDBManager,
    _meta_data: &MetaDataSet,
) -> () {
    print_utils::dbg_print("update_assets_metadata");
}

pub(crate) async fn update_instrument_metadata(
    _dbm_mddb: &PostgresMDDBManager,
    _meta_data: &MetaDataSet,
) -> () {
    print_utils::dbg_print("update_assets_metadata");
}
