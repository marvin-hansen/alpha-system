use crate::print_utils;
use crate::workflow;
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

pub(crate) async fn update_all_metadata(dbm_mddb: &PostgresMDDBManager, meta_data: &MetaDataSet) {
    print_utils::dbg_print("update_all_metadata");

    workflow::update_assets_metadata(dbm_mddb, meta_data).await;

    workflow::update_exchanges_metadata(dbm_mddb, meta_data).await;

    workflow::update_instruments_metadata(dbm_mddb, meta_data).await;
}
