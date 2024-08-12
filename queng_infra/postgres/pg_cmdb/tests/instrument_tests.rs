use crate::util;
use pg_cmdb::model::instrument::{CreateInstrument, Instrument, UpdateInstrument};

#[test]
fn test_instrument() {
    let pool = util::postgres_connection_pool();

    let create_instrument = CreateInstrument {
        code: "test_code".to_string(),
        class: "test_class".to_string(),
        exchange_code: "test_exchange_code".to_string(),
        exchange_pair_code: "test_exchange_pair_code".to_string(),
        base_asset: "test_base_asset".to_string(),
        quote_asset: "test_quote_asset".to_string(),
        instrument_figi: Some("test".to_string()),
    };

    let mut conn = &mut pool.get().unwrap();

    let result = Instrument::create(&mut conn, &create_instrument);
    assert!(result.is_ok());

    let instrument = result.unwrap();

    assert_eq!(instrument.code, "test_code");
    assert_eq!(instrument.class, "test_class");
    assert_eq!(instrument.exchange_code, "test_exchange_code");
    assert_eq!(instrument.exchange_pair_code, "test_exchange_pair_code");
    assert_eq!(instrument.base_asset, "test_base_asset");
    assert_eq!(instrument.quote_asset, "test_quote_asset");
    assert_eq!(instrument.instrument_figi, Some("test".to_string()));

    let update = UpdateInstrument::new(
        Some("new_test_class".to_string()),
        None,
        None,
        None,
        None,
        None,
    );

    let result = Instrument::check_if_instrument_code_exists(conn, "test_code".to_string());
    assert!(result.is_ok());
    assert!(result.unwrap());

    let result = Instrument::update(&mut conn, "test_code".to_string(), &update);
    assert!(result.is_ok());

    let result = Instrument::read(&mut conn, "test_code".to_string());
    assert!(result.is_ok());

    let instrument = result.unwrap();

    assert_eq!(instrument.code, "test_code");
    assert_eq!(instrument.class, "new_test_class");
    assert_eq!(instrument.exchange_code, "test_exchange_code");
    assert_eq!(instrument.exchange_pair_code, "test_exchange_pair_code");
    assert_eq!(instrument.base_asset, "test_base_asset");
    assert_eq!(instrument.quote_asset, "test_quote_asset");
    assert_eq!(instrument.instrument_figi, Some("test".to_string()));

    let result = Instrument::read_all(&mut conn);
    assert!(result.is_ok());

    let all_instruments = result.unwrap();
    assert!(all_instruments.len() > 0);

    let result = Instrument::read(&mut conn, "test_code".to_string());
    assert!(result.is_ok());

    let result = Instrument::delete(&mut conn, "test_code".to_string());
    assert!(result.is_ok());

    let result = Instrument::check_if_instrument_code_exists(conn, "test_code".to_string());
    assert!(result.is_ok());
    assert!(!result.unwrap());
}
