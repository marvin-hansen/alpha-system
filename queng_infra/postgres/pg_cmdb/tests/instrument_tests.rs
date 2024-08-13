use common_exchange::prelude::Instrument as CommonInstrument;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use pg_cmdb::model::instrument::{Instrument, UpdateInstrument};
use std::env;

fn postgres_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .or_else(|_| env::var("POSTGRES_DATABASE_URL"))
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

#[test]
fn test_instrument() {
    let pool = postgres_connection_pool();
    let conn = &mut pool.get().unwrap();

    println!("Test create!");
    test_create_instrument(conn);

    println!("Test count!");
    test_count_instrument(conn);

    println!("Test check_if_instrument_code_exists!");
    test_check_if_instrument_code_exists(conn);

    println!("Test read!");
    test_read_instrument(conn);

    println!("Test read_all!");
    test_read_all_instruments(conn);

    println!("Test update!");
    test_update_instrument(conn);

    println!("Test delete!");
    test_delete_instrument(conn);
}

fn test_create_instrument(conn: &mut pg_cmdb::Connection) {
    let instrument = CommonInstrument::new(
        "test_code".to_string(),
        "test_class".to_string(),
        "test_exchange_code".to_string(),
        "test_exchange_pair_code".to_string(),
        "test_base_asset".to_string(),
        "test_quote_asset".to_string(),
        Some("test".to_string()),
    );

    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    let instrument = result.unwrap();

    assert_eq!(instrument.code(), "test_code");
    assert_eq!(instrument.class(), "test_class");
    assert_eq!(instrument.exchange_code(), "test_exchange_code");
    assert_eq!(instrument.exchange_pair_code(), "test_exchange_pair_code");
    assert_eq!(instrument.base_asset(), "test_base_asset");
    assert_eq!(instrument.quote_asset(), "test_quote_asset");
    assert_eq!(instrument.instrument_figi(), &Some("test".to_string()));
}

fn test_count_instrument(conn: &mut pg_cmdb::Connection) {
    let result = Instrument::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

fn test_check_if_instrument_code_exists(conn: &mut pg_cmdb::Connection) {
    let result = Instrument::check_if_instrument_code_exists(conn, "test_code".to_string());
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

fn test_read_instrument(conn: &mut pg_cmdb::Connection) {
    let result = Instrument::read(conn, "test_code".to_string());
    // dbg!(&result);

    assert!(result.is_ok());
    let instrument = result.unwrap();

    assert_eq!(instrument.code(), "test_code");
    assert_eq!(instrument.class(), "test_class");
    assert_eq!(instrument.exchange_code(), "test_exchange_code");
    assert_eq!(instrument.exchange_pair_code(), "test_exchange_pair_code");
    assert_eq!(instrument.base_asset(), "test_base_asset");
    assert_eq!(instrument.quote_asset(), "test_quote_asset");
    assert_eq!(instrument.instrument_figi(), &Some("test".to_string()));
}

fn test_read_all_instruments(conn: &mut pg_cmdb::Connection) {
    let result = Instrument::read_all(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let all_instruments = result.unwrap();
    assert!(all_instruments.len() > 0);
}

fn test_update_instrument(conn: &mut pg_cmdb::Connection) {
    let update = UpdateInstrument::new(
        Some("new_test_class".to_string()),
        None,
        None,
        None,
        None,
        None,
    );

    let result = Instrument::update(conn, "test_code".to_string(), &update);
    // dbg!(&result);
    assert!(result.is_ok());
}

fn test_delete_instrument(conn: &mut pg_cmdb::Connection) {
    let result = Instrument::check_if_instrument_code_exists(conn, "test_code".to_string());
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());

    let result = Instrument::delete(conn, "test_code".to_string());
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Instrument::check_if_instrument_code_exists(conn, "test_code".to_string());
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}
