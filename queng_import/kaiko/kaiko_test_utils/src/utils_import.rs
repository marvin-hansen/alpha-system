/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::utils_shared;
use common_metadata::MetaDataSet;
// Note, full and partial import test may run concurrently hence require different testdata
// to ensure zero conflicts can happen in the DB.

#[must_use]
pub fn get_partial_import_test_asset_id() -> String {
    "test_asset_code".to_string()
}

#[must_use]
pub fn get_partial_import_test_exchange_id() -> String {
    "test_exchange_code".to_string()
}

#[must_use]
pub fn get_partial_import_test_instrument_id() -> String {
    "test_exchange_code_currency_btc_usd".to_string()
}

#[must_use]
pub fn get_partial_import_test_data_set() -> MetaDataSet {
    let assets = vec![utils_shared::get_test_asset(
        get_partial_import_test_asset_id(),
    )];
    let exchanges = vec![utils_shared::get_test_meta_exchange(
        get_partial_import_test_exchange_id(),
    )];
    let instruments = vec![utils_shared::get_test_meta_instrument(
        "BTC".to_string(),
        "USD".to_string(),
        get_partial_import_test_asset_id(),
    )];

    MetaDataSet::new(assets, exchanges, instruments)
}

#[must_use]
pub fn get_full_import_test_asset_id() -> String {
    "full_test_asset_code".to_string()
}

#[must_use]
pub fn get_full_import_test_exchange_id() -> String {
    "full_test_exchange_code".to_string()
}

#[must_use]
pub fn get_full_import_test_instrument_id() -> String {
    "test_exchange_code_currency_eth_eur".to_string()
}

#[must_use]
pub fn get_full_import_test_data_set() -> MetaDataSet {
    let assets = vec![utils_shared::get_test_asset(get_full_import_test_asset_id())];
    let exchanges = vec![utils_shared::get_test_meta_exchange(
        get_full_import_test_exchange_id(),
    )];
    let instruments = vec![utils_shared::get_test_meta_instrument(
        "ETH".to_string(),
        "EUR".to_string(),
        get_full_import_test_asset_id(),
    )];

    MetaDataSet::new(assets, exchanges, instruments)
}
