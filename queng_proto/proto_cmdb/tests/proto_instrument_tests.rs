use proto_cmdb::proto::ProtoInstrument;

#[test]
fn test_proto_instrument() {
    let proto = ProtoInstrument {
        instrument_code: "test_code".to_string(),
        exchange_code: "test_exchange_code".to_string(),
        exchange_pair_code: "test_exchange_pair_code".to_string(),
        base_asset: "test_base_asset".to_string(),
        quote_asset: "test_quote_asset".to_string(),
        instrument_figi: Some("test".to_string()),
        instrument_class: "test_class".to_string(),
    };

    assert_eq!(proto.instrument_code, "test_code");
    assert_eq!(proto.exchange_code, "test_exchange_code");
    assert_eq!(proto.exchange_pair_code, "test_exchange_pair_code");
    assert_eq!(proto.base_asset, "test_base_asset");
    assert_eq!(proto.quote_asset, "test_quote_asset");
    assert_eq!(proto.instrument_figi, Some("test".to_string()));
    assert_eq!(proto.instrument_class, "test_class");
}
