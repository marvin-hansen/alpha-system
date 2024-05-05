use common::prelude::{Instrument, InstrumentMetadata, InstrumentsRoot};

#[test]
fn test_instruments_root_and_instrument_properties() {
    // Construct an InstrumentMetadata
    let metadata = InstrumentMetadata {
        pair_figi: Some("BBG000BLNNH6".to_string()),
        instrument_figi: Some("BBG000BLNNH7".to_string()),
    };

    // Construct an Instrument
    let instrument = Instrument {
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
    };

    // Construct an InstrumentsRoot
    let instruments_root = InstrumentsRoot {
        result: "success".to_string(),
        data: vec![instrument],
    };

    // Test InstrumentsRoot properties
    assert_eq!(instruments_root.result, "success");
    assert_eq!(instruments_root.data.len(), 1);

    // Test properties of Instrument
    let test_instrument = &instruments_root.data[0];
    assert_eq!(test_instrument.kaiko_legacy_exchange_slug, "kaiko-exchange");
    assert_eq!(
        test_instrument.trade_start_time,
        Some("2021-01-01T00:00:00Z".to_string())
    );
    assert_eq!(
        test_instrument.trade_end_time,
        Some("2021-12-31T23:59:59Z".to_string())
    );
    assert_eq!(test_instrument.exchange_code, "XKRX");
    assert_eq!(test_instrument.exchange_pair_code, "BTCUSD");
    assert_eq!(test_instrument.base_asset, "BTC");
    assert_eq!(test_instrument.quote_asset, "USD");
    assert_eq!(test_instrument.kaiko_legacy_symbol, "BTCUSD");
    assert_eq!(test_instrument.code, "BTC-USD");
    assert_eq!(test_instrument.class, "currency");
    assert_eq!(test_instrument.trade_start_timestamp, Some(1609459200));
    assert_eq!(test_instrument.trade_end_timestamp, Some(1640995199));
    assert_eq!(test_instrument.trade_compressed_size, 1024);
    assert_eq!(test_instrument.trade_count, 10000);

    // Test properties of InstrumentMetadata
    let test_metadata = test_instrument.metadata.as_ref().unwrap();
    assert_eq!(test_metadata.pair_figi, Some("BBG000BLNNH6".to_string()));
    assert_eq!(
        test_metadata.instrument_figi,
        Some("BBG000BLNNH7".to_string())
    );
}
