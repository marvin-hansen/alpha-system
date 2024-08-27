use common_exchange::prelude::Instrument as CommonInstrument;
use container_specs::postgres_container_specs::postgres_db_container_config;
use diesel::{Connection, PgConnection};
use docker_utils::DockerUtil;
use pg_cmdb::model::instrument::Instrument;
use pg_cmdb::run_cmdb_db_migration;

async fn start_docker() {
    // Create new DockerUtil
    let docker_util = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Initiate CI container
    let container_config = postgres_db_container_config();
    docker_util
        .setup_container(&container_config)
        .await
        .expect("Failed to setup ci api proxy container");
}

fn postgres_connection() -> PgConnection {
    let database_url = "postgres://postgres:postgres@localhost/postgres";

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn run_db_migration(conn: &mut PgConnection) {
    let result = run_cmdb_db_migration(conn);
    //dbg!(&result);
    assert!(result.is_ok());
}

async fn test_setup() {
    start_docker().await;

    let conn = &mut postgres_connection();

    run_db_migration(conn);
}

#[tokio::test]
async fn test_create_instrument() {
    test_setup().await;

    let mut connection = postgres_connection();
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let instrument = get_instrument();

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

#[tokio::test]
async fn test_count_instrument() {
    test_setup().await;

    let mut connection = postgres_connection();
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let result = Instrument::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    let instrument = get_instrument();
    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Instrument::count(conn);
    // dbg!(&result);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[tokio::test]
async fn test_check_if_instrument_code_exists() {
    test_setup().await;

    let mut connection = postgres_connection();
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let instrument = get_instrument();
    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Instrument::check_if_instrument_code_exists(conn, "test_code".to_string());
    // dbg!(&result);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_read_instrument() {
    test_setup().await;

    let mut connection = postgres_connection();
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let instrument = get_instrument();
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

#[tokio::test]
async fn test_read_all_instruments() {
    test_setup().await;

    let mut connection = postgres_connection();
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let instrument = get_instrument();
    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    let result = Instrument::read_all(conn);
    // dbg!(&result);
    assert!(result.is_ok());

    let all_instruments = result.unwrap();
    assert!(all_instruments.len() > 0);
}

#[tokio::test]
async fn test_update_instrument() {
    test_setup().await;

    let mut connection = postgres_connection();
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let instrument = get_instrument();
    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

    let update = CommonInstrument::new(
        "test_code".to_string(),
        "new_test_class".to_string(),
        "test_exchange_code".to_string(),
        "test_exchange_pair_code".to_string(),
        "test_base_asset".to_string(),
        "test_quote_asset".to_string(),
        Some("test".to_string()),
    );

    let result = Instrument::update(conn, "test_code".to_string(), &update);
    // dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_instrument() {
    test_setup().await;

    let mut connection = postgres_connection();
    let conn = &mut connection;
    conn.begin_test_transaction()
        .expect("Failed to begin test transaction");

    let instrument = get_instrument();
    let result = Instrument::create(conn, &instrument);
    // dbg!(&result);
    assert!(result.is_ok());

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

fn get_instrument() -> CommonInstrument {
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
