use crate::utils_shared;
use common_metadata::MetaDataSet;

#[must_use]
pub fn get_partial_update_test_asset_id() -> String {
    "test_partial_update_asset_code".to_string()
}

#[must_use]
pub fn get_partial_update_test_exchange_id() -> String {
    "test_partial_update_exchange_code".to_string()
}

#[must_use]
pub fn get_partial_update_test_instrument_id() -> String {
    "test_partial_update_exchange_code_currency_btc_usd".to_string()
}

#[must_use]
pub fn get_pre_update_test_data_set() -> MetaDataSet {
    let assets = vec![utils_shared::get_test_asset(
        get_partial_update_test_asset_id(),
    )];
    let exchanges = vec![utils_shared::get_test_meta_exchange(
        get_partial_update_test_exchange_id(),
    )];
    let instruments = vec![utils_shared::get_test_meta_instrument(
        "BTC".to_string(),
        "USD".to_string(),
        get_partial_update_test_exchange_id(),
    )];

    MetaDataSet::new(assets, exchanges, instruments)
}

#[must_use]
pub fn get_update_test_data_set() -> MetaDataSet {
    let assets = vec![utils_shared::get_test_update_asset(
        get_partial_update_test_asset_id(),
    )];
    let exchanges = vec![utils_shared::get_test_update_meta_exchange(
        get_partial_update_test_exchange_id(),
    )];
    let instruments = vec![utils_shared::get_test_update_meta_instrument(
        "BTC".to_string(),
        "USD".to_string(),
        get_partial_update_test_exchange_id(),
    )];

    MetaDataSet::new(assets, exchanges, instruments)
}
