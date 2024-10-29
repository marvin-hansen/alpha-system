use common_metadata::prelude::{
    InstrumentMetadata, MetaAsset, MetaDataSet, MetaExchange, MetaInstrument,
};

// Note, full and partial import test may run concurrently hence require different testdata
// to ensure zero conflicts can happen in the DB.

pub fn get_partial_test_asset_id() -> String {
    "test_asset_code".to_string()
}

pub fn get_partial_test_exchange_id() -> String {
    "test_exchange_code".to_string()
}

pub fn get_partial_test_instrument_id() -> String {
    "test_exchange_code_currency_btc_usd".to_string()
}

pub fn get_partial_test_data_set() -> MetaDataSet {
    let assets = vec![get_test_asset(get_partial_test_asset_id())];
    let exchanges = vec![get_test_meta_exchange(get_partial_test_exchange_id())];
    let instruments = vec![get_test_meta_instrument(
        "BTC".to_string(),
        "USD".to_string(),
    )];

    MetaDataSet::new(assets, exchanges, instruments)
}

pub fn get_full_test_asset_id() -> String {
    "full_test_asset_code".to_string()
}

pub fn get_full_test_exchange_id() -> String {
    "full_test_exchange_code".to_string()
}

pub fn get_full_test_instrument_id() -> String {
    "test_exchange_code_currency_eth_eur".to_string()
}

pub fn get_full_test_data_set() -> MetaDataSet {
    let assets = vec![get_test_asset(get_full_test_asset_id())];
    let exchanges = vec![get_test_meta_exchange(get_full_test_exchange_id())];
    let instruments = vec![get_test_meta_instrument(
        "ETH".to_string(),
        "EUR".to_string(),
    )];

    MetaDataSet::new(assets, exchanges, instruments)
}

fn get_test_asset(code: String) -> MetaAsset {
    MetaAsset {
        code,
        name: "test_asset_name".to_string(),
        asset_classes: vec![],
        asset_class: "crypto".to_string(),
        metadata: None,
        addresses: None,
    }
}

fn get_test_meta_exchange(code: String) -> MetaExchange {
    MetaExchange {
        code,
        name: "test_exchange_name".to_string(),
        kaiko_legacy_slug: "test_kaiko_legacy_slug".to_string(),
    }
}

fn get_test_meta_instrument(base_asset: String, quote_asset: String) -> MetaInstrument {
    let metadata = InstrumentMetadata {
        pair_figi: Some("BBG000BLNNH6".to_string()),
        instrument_figi: Some("BBG000BLNNH7".to_string()),
    };

    MetaInstrument {
        kaiko_legacy_exchange_slug: "kaiko-exchange".to_string(),
        trade_start_time: Some("2021-01-01T00:00:00Z".to_string()),
        trade_end_time: Some("2021-12-31T23:59:59Z".to_string()),
        exchange_code: get_partial_test_exchange_id(),
        exchange_pair_code: "BTCUSD".to_string(),
        base_asset,
        quote_asset,
        kaiko_legacy_symbol: "BTCUSD".to_string(),
        code: "BTC-USD".to_string(),
        class: "currency".to_string(),
        metadata: Some(metadata),
        trade_start_timestamp: Some(1609459200),
        trade_end_timestamp: Some(1640995199),
        trade_compressed_size: 1024,
        trade_count: 10000,
    }
}
