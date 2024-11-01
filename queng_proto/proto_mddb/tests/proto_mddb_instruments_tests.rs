use proto_mddb::proto::ProtoMetaInstrument;

#[test]
fn test_proto_meta_instrument() {
    let instrument = ProtoMetaInstrument {
        instrument_id: "instrument_id".to_string(),
        instrument_code: "instrument_code".to_string(),
        instrument_hash: "instrument_hash".to_string(),
        instrument_class: "instrument_class".to_string(),
        instrument_base_asset: "instrument_base_asset".to_string(),
        instrument_quote_asset: "instrument_quote_asset".to_string(),
        instrument_exchanges_code: "instrument_exchanges_code".to_string(),
        instrument_exchange_pair_code: "instrument_exchange_pair_code".to_string(),
        instrument_pair_figi: Some("instrument_pair_figi".to_string()),
        instrument_figi: Some("instrument_figi".to_string()),
        instrument_trade_start_timestamp: None,
        instrument_trade_end_timestamp: None,
    };

    assert_eq!(instrument.instrument_id, "instrument_id");
    assert_eq!(instrument.instrument_code, "instrument_code");
    assert_eq!(instrument.instrument_hash, "instrument_hash");
    assert_eq!(instrument.instrument_class, "instrument_class");
    assert_eq!(instrument.instrument_base_asset, "instrument_base_asset");
    assert_eq!(instrument.instrument_quote_asset, "instrument_quote_asset");
    assert_eq!(
        instrument.instrument_exchanges_code,
        "instrument_exchanges_code"
    );
    assert_eq!(
        instrument.instrument_exchange_pair_code,
        "instrument_exchange_pair_code"
    );
    assert_eq!(
        instrument.instrument_pair_figi,
        Some("instrument_pair_figi".to_string())
    );
    assert_eq!(
        instrument.instrument_figi,
        Some("instrument_figi".to_string())
    );
    assert_eq!(instrument.instrument_trade_start_timestamp, None);
    assert_eq!(instrument.instrument_trade_end_timestamp, None);
}
