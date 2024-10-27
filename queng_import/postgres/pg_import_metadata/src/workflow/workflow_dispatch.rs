use crate::print_utils;
use common_metadata::prelude::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

pub(crate) async fn workflow_dispatch(meta_data: &MetaDataSet, dbm_mddb: &PostgresMDDBManager) {
    println!("workflow_dispatch");

    let stats = meta_data.stats();
    let expected_asset_count = stats.number_assets() as usize;
    let expected_exchange_count = stats.number_exchanges() as usize;
    let expected_instrument_count = stats.number_instruments() as usize;

    print_utils::dbg_print("Count assets in database");
    let db_asset_count = dbm_mddb
        .count_assets()
        .await
        .expect("Failed to count assets") as usize;

    print_utils::dbg_print("Count exchanges in database");
    let db_exchange_count = dbm_mddb
        .count_exchanges()
        .await
        .expect("Failed to count exchanges") as usize;

    print_utils::dbg_print("Count instruments in database");
    let db_instrument_count = dbm_mddb
        .count_instruments()
        .await
        .expect("Failed to count instruments") as usize;

    if db_asset_count == expected_asset_count {
        print_utils::dbg_print("Assets already imported");
    }

    if db_exchange_count == expected_exchange_count {
        print_utils::dbg_print("Exchanges already imported");
    }

    if db_instrument_count == expected_instrument_count {
        print_utils::dbg_print("Instruments already imported");
    }
}
