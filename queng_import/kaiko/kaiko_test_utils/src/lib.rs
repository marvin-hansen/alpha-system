use common_metadata::prelude::{
    InstrumentMetadata, MetaAsset, MetaDataSet, MetaExchange, MetaInstrument,
};

pub fn get_test_asset_id() -> String {
    "test_asset_code".to_string()
}

pub fn get_test_exchange_id() -> String {
    "test_exchange_code".to_string()
}

pub fn get_test_instrument_id() -> String {
    "BTC-USD".to_string()
}

pub fn get_test_meta_data_set() -> MetaDataSet {
    let assets = vec![get_test_asset()];
    let exchanges = vec![get_test_meta_exchange()];
    let instruments = vec![get_test_meta_instrument()];

    MetaDataSet::new(assets, exchanges, instruments)
}

fn get_test_asset() -> MetaAsset {
    MetaAsset {
        code: get_test_asset_id(),
        name: "test_asset_name".to_string(),
        asset_classes: vec![],
        asset_class: "crypto".to_string(),
        metadata: None,
        addresses: None,
    }
}

fn get_test_meta_exchange() -> MetaExchange {
    MetaExchange {
        code: get_test_exchange_id(),
        name: "test_exchange_name".to_string(),
        kaiko_legacy_slug: "test_kaiko_legacy_slug".to_string(),
    }
}

fn get_test_meta_instrument() -> MetaInstrument {
    let metadata = InstrumentMetadata {
        pair_figi: Some("BBG000BLNNH6".to_string()),
        instrument_figi: Some("BBG000BLNNH7".to_string()),
    };

    MetaInstrument {
        kaiko_legacy_exchange_slug: "kaiko-exchange".to_string(),
        trade_start_time: Some("2021-01-01T00:00:00Z".to_string()),
        trade_end_time: Some("2021-12-31T23:59:59Z".to_string()),
        exchange_code: get_test_exchange_id(),
        exchange_pair_code: "BTCUSD".to_string(),
        base_asset: "BTC".to_string(),
        quote_asset: "USD".to_string(),
        kaiko_legacy_symbol: "BTCUSD".to_string(),
        code: get_test_instrument_id(),
        class: "currency".to_string(),
        metadata: Some(metadata),
        trade_start_timestamp: Some(1609459200),
        trade_end_timestamp: Some(1640995199),
        trade_compressed_size: 1024,
        trade_count: 10000,
    }
}
