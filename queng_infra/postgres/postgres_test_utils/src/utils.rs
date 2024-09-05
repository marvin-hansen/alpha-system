use common_config::prelude::{ServiceConfig, ServiceID};
use common_exchange::prelude::Instrument as CommonInstrument;
use common_exchange::prelude::{AccountType, PortfolioConfig as CommonPortfolioConfig};
use common_metadata::prelude::{InstrumentMetadata, MetaExchange, MetaInstrument, MetaStats};
use pg_metadb::prelude::Asset;

pub fn get_test_asset() -> Asset {
    Asset {
        asset_code: "test_asset_code".to_string(),
        asset_name: "test_asset_name".to_string(),
        asset_classes: vec![],
        asset_figi: None,
    }
}

pub fn get_test_meta_exchange() -> MetaExchange {
    MetaExchange {
        code: "test_exchange_code".to_string(),
        name: "test_exchange_name".to_string(),
        kaiko_legacy_slug: "test_kaiko_legacy_slug".to_string(),
    }
}
pub fn get_test_meta_instrument() -> MetaInstrument {
    let metadata = InstrumentMetadata {
        pair_figi: Some("BBG000BLNNH6".to_string()),
        instrument_figi: Some("BBG000BLNNH7".to_string()),
    };

    MetaInstrument {
        kaiko_legacy_exchange_slug: "kaiko-exchange".to_string(),
        trade_start_time: Some("2021-01-01T00:00:00Z".to_string()),
        trade_end_time: Some("2021-12-31T23:59:59Z".to_string()),
        exchange_code: "XKRX".to_string(),
        exchange_pair_code: "BTCUSD".to_string(),
        base_asset: "BTC".to_string(),
        quote_asset: "USD".to_string(),
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

pub fn get_test_meta_stats() -> MetaStats {
    let download_timestamp = "2023-10-01T12:00:00Z".to_string();
    let hash = "abc123".to_string();
    let number_assets = 100;
    let number_exchanges = 10;
    let number_instruments = 50;

    MetaStats::new(
        download_timestamp.clone(),
        hash.clone(),
        number_assets,
        number_exchanges,
        number_instruments,
    )
}

pub fn get_test_instrument() -> CommonInstrument {
    CommonInstrument::new(
        "test_code".to_string(),
        "test_class".to_string(),
        "test_exchange_code".to_string(),
        "test_exchange_pair_code".to_string(),
        "test_base_asset".to_string(),
        "test_quote_asset".to_string(),
        Some("test".to_string()),
    )
}

pub fn get_test_portfolio() -> CommonPortfolioConfig {
    CommonPortfolioConfig::new(
        1,
        "Test Portfolio".to_string(),
        AccountType::Spot,
        "12345".to_string(),
        "USD".to_string(),
        1000.0,
        500.0,
        20.0,
        Vec::new(),
        30.0,
        10.0,
        500.0,
        1000.0,
        50.0,
        100.0,
    )
}

pub fn get_test_service_config() -> ServiceConfig {
    ServiceConfig::new(
        ServiceID::SMDB,
        "name".to_string(),
        1,
        true,
        "description".to_string(),
        "health_check_uri".to_string(),
        "base_uri".to_string(),
        vec![ServiceID::DBGW],
        Vec::from([
            common_config::prelude::Endpoint::default(),
            common_config::prelude::Endpoint::default(),
        ]),
    )
}
