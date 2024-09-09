use common_metadata::prelude::MetaInstrument;
use pg_mddb::prelude::Instrument;

#[test]
fn test_from_meta_instrument() {
    let meta_instrument = MetaInstrument {
        kaiko_legacy_exchange_slug: String::new(),
        trade_start_time: None,
        trade_end_time: None,
        exchange_code: "exchange_code".to_string(),
        exchange_pair_code: "exchange_pair_code".to_string(),
        base_asset: "base_asset".to_string(),
        quote_asset: "quote_asset".to_string(),
        kaiko_legacy_symbol: String::new(),
        code: "instrument_code".to_string(),
        class: "instrument_class".to_string(),
        metadata: None,
        trade_start_timestamp: Some(1622548800),
        trade_end_timestamp: Some(1625130800),
        trade_compressed_size: 0,
        trade_count: 0,
    };

    let postgres_instrument = Instrument::from_meta_instrument(meta_instrument.clone());

    assert_eq!(postgres_instrument.instrument_id, meta_instrument.code);
    assert_eq!(postgres_instrument.instrument_class, meta_instrument.class);
    assert_eq!(
        postgres_instrument.instrument_base_asset,
        meta_instrument.base_asset
    );
    assert_eq!(
        postgres_instrument.instrument_quote_asset,
        meta_instrument.quote_asset
    );
    assert_eq!(
        postgres_instrument.instrument_exchanges_code,
        meta_instrument.exchange_code
    );
    assert_eq!(
        postgres_instrument.instrument_exchange_pair_code,
        meta_instrument.exchange_pair_code
    );
    assert_eq!(
        postgres_instrument.instrument_trade_start_timestamp,
        meta_instrument.trade_start_timestamp.map(|ts| ts as i64)
    );
    assert_eq!(
        postgres_instrument.instrument_trade_end_timestamp,
        meta_instrument.trade_end_timestamp
    );
}

#[test]
fn test_to_meta_instrument() {
    let postgres_instrument = Instrument {
        instrument_id: "instrument_code".to_string(),
        instrument_class: "instrument_class".to_string(),
        instrument_base_asset: "base_asset".to_string(),
        instrument_quote_asset: "quote_asset".to_string(),
        instrument_exchanges_code: "exchange_code".to_string(),
        instrument_exchange_pair_code: "exchange_pair_code".to_string(),
        instrument_pair_figi: None,
        instrument_figi: None,
        instrument_trade_start_timestamp: Some(1622548800),
        instrument_trade_end_timestamp: Some(1625130800),
    };

    let meta_instrument = postgres_instrument.to_meta_instrument();

    assert_eq!(meta_instrument.code, postgres_instrument.instrument_id);
    assert_eq!(meta_instrument.class, postgres_instrument.instrument_class);
    assert_eq!(
        meta_instrument.base_asset,
        postgres_instrument.instrument_base_asset
    );
    assert_eq!(
        meta_instrument.quote_asset,
        postgres_instrument.instrument_quote_asset
    );
    assert_eq!(
        meta_instrument.exchange_code,
        postgres_instrument.instrument_exchanges_code
    );
    assert_eq!(
        meta_instrument.exchange_pair_code,
        postgres_instrument.instrument_exchange_pair_code
    );
    assert_eq!(meta_instrument.metadata, None);
    assert_eq!(
        meta_instrument.trade_start_timestamp,
        postgres_instrument
            .instrument_trade_start_timestamp
            .map(|ts| ts as u64)
    );
    assert_eq!(
        meta_instrument.trade_end_timestamp,
        postgres_instrument.instrument_trade_end_timestamp
    );
}
