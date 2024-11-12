use common_metadata::prelude::{InstrumentMetadata, MetaInstrument};
use proto_mddb::proto::*;
use proto_mddb_utils::prelude::*;

fn create_test_meta_instrument() -> MetaInstrument {
    MetaInstrument {
        exchange_code: "XKRX".to_string(),
        exchange_pair_code: "BTCUSD".to_string(),
        base_asset: "BTC".to_string(),
        quote_asset: "USD".to_string(),
        code: "BTC-USD".to_string(),
        class: "currency".to_string(),
        metadata: Some(InstrumentMetadata {
            pair_figi: Some("BBG000BLNNH6".to_string()),
            instrument_figi: Some("BBG000BLNNH7".to_string()),
        }),
        trade_start_timestamp: Some(1609459200),
        trade_end_timestamp: Some(1640995199),
        trade_compressed_size: 1024,
        trade_count: 10000,
        kaiko_legacy_exchange_slug: "".to_string(),
        trade_start_time: None,
        trade_end_time: None,
        kaiko_legacy_symbol: "".to_string(),
    }
}

#[test]
fn test_get_count_instruments_request() {
    let request = get_count_instruments_request();
    assert!(matches!(request, CountInstrumentsRequest {}));
}

#[test]
fn test_get_check_if_instrument_exists_request() {
    let instrument_id = "BTC-USD-XKRX";
    let request = get_check_if_instrument_exists_request(instrument_id);
    assert_eq!(request.instrument_id, instrument_id);
}

#[test]
fn test_get_instrument_by_id_request() {
    let instrument_id = "BTC-USD-XKRX";
    let request = get_instrument_by_id_request(instrument_id);
    assert_eq!(request.instrument_id, instrument_id);
}

#[test]
fn test_get_instrument_by_figi_request() {
    let figi = "BBG000BLNNH6";
    let request = get_instrument_by_figi_request(figi);
    assert_eq!(request.instrument_figi, figi);
}

#[test]
fn test_get_all_instruments_request() {
    let request = get_all_instruments_request();
    assert!(matches!(request, GetAllInstrumentsRequest {}));
}

#[test]
fn test_get_all_instruments_for_base_asset_request() {
    let base_asset = "BTC";
    let request = get_all_instruments_for_base_asset_request(base_asset);
    assert_eq!(request.base_asset, base_asset);
}

#[test]
fn test_get_count_instruments_response() {
    let count = 42;
    let response = get_count_instruments_response(count);
    assert_eq!(response.count, count);
}

#[test]
fn test_get_check_if_instrument_exists_response() {
    let instrument_id = "BTC-USD-XKRX";
    let exists = true;
    let response = get_check_if_instrument_exists_response(instrument_id, exists);
    assert_eq!(response.instrument_id, instrument_id);
    assert_eq!(response.exists, exists);
}

#[test]
fn test_get_instrument_by_id_response() {
    let meta_instrument = create_test_meta_instrument();
    let response = get_instrument_by_id_response(Some(meta_instrument.clone()));
    assert!(response.instrument.is_some());
    let proto_instrument = response.instrument.unwrap();
    assert_eq!(
        proto_instrument.instrument_base_asset,
        meta_instrument.base_asset
    );
    assert_eq!(
        proto_instrument.instrument_quote_asset,
        meta_instrument.quote_asset
    );
}

#[test]
fn test_get_all_instruments_response() {
    let meta_instruments = vec![create_test_meta_instrument()];
    let response = get_all_instruments_response(meta_instruments);
    assert_eq!(response.instruments.len(), 1);
    let proto_instrument = &response.instruments[0];
    assert_eq!(proto_instrument.instrument_exchanges_code, "XKRX");
    assert_eq!(proto_instrument.instrument_base_asset, "BTC");
}

#[test]
fn test_meta_instrument_to_proto_instrument() {
    let meta_instrument = create_test_meta_instrument();
    let proto_instrument = meta_instrument_to_proto_instrument(&meta_instrument);

    assert_eq!(
        proto_instrument.instrument_base_asset,
        meta_instrument.base_asset
    );
    assert_eq!(
        proto_instrument.instrument_quote_asset,
        meta_instrument.quote_asset
    );
    assert_eq!(
        proto_instrument.instrument_exchanges_code,
        meta_instrument.exchange_code
    );
    assert_eq!(proto_instrument.instrument_class, meta_instrument.class);
    assert_eq!(
        proto_instrument.instrument_pair_figi,
        Some("BBG000BLNNH6".to_string())
    );
    assert_eq!(
        proto_instrument.instrument_figi,
        Some("BBG000BLNNH7".to_string())
    );
}

#[test]
fn test_proto_instrument_to_meta_instrument() {
    let original = create_test_meta_instrument();
    let proto = meta_instrument_to_proto_instrument(&original);
    let converted = proto_instrument_to_meta_instrument(&proto);

    assert_eq!(converted.base_asset, original.base_asset);
    assert_eq!(converted.quote_asset, original.quote_asset);
    assert_eq!(converted.exchange_code, original.exchange_code);
    assert_eq!(converted.class, original.class);
    assert_eq!(
        converted.metadata.as_ref().unwrap().pair_figi,
        original.metadata.as_ref().unwrap().pair_figi
    );
}
