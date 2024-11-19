use common_metadata::{InstrumentMetadata, MetaAsset, MetaExchange, MetaInstrument};

pub fn get_test_asset(code: String) -> MetaAsset {
    MetaAsset {
        code,
        name: "test_asset_name".to_string(),
        asset_classes: vec![],
        asset_class: "crypto".to_string(),
        metadata: None,
        addresses: None,
    }
}

pub fn get_test_update_asset(code: String) -> MetaAsset {
    MetaAsset {
        code,
        name: "test_asset_name".to_string(),
        asset_classes: vec!["Class1".to_string(), "Class2".to_string()],
        asset_class: "crypto".to_string(),
        metadata: None,
        addresses: None,
    }
}

pub fn get_test_meta_exchange(code: String) -> MetaExchange {
    MetaExchange {
        code,
        name: "test_exchange_name".to_string(),
        kaiko_legacy_slug: "test_kaiko_legacy_slug".to_string(),
    }
}

pub fn get_test_update_meta_exchange(code: String) -> MetaExchange {
    MetaExchange {
        code,
        name: "updated_test_exchange_name".to_string(),
        kaiko_legacy_slug: "updated_test_kaiko_legacy_slug".to_string(),
    }
}

pub fn get_test_meta_instrument(
    base_asset: String,
    quote_asset: String,
    exchange_code: String,
) -> MetaInstrument {
    let metadata = InstrumentMetadata {
        pair_figi: None,
        instrument_figi: None,
    };

    MetaInstrument {
        kaiko_legacy_exchange_slug: "kaiko-exchange".to_string(),
        trade_start_time: Some("2021-01-01T00:00:00Z".to_string()),
        trade_end_time: None,
        exchange_code,
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

pub fn get_test_update_meta_instrument(
    base_asset: String,
    quote_asset: String,
    exchange_code: String,
) -> MetaInstrument {
    let metadata = InstrumentMetadata {
        pair_figi: Some("BBG000BLNNH6".to_string()),
        instrument_figi: Some("BBG000BLNNH7".to_string()),
    };

    MetaInstrument {
        kaiko_legacy_exchange_slug: "kaiko-exchange".to_string(),
        trade_start_time: Some("2021-01-01T00:00:00Z".to_string()),
        trade_end_time: Some("2024-10-31T23:59:59Z".to_string()),
        exchange_code,
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
