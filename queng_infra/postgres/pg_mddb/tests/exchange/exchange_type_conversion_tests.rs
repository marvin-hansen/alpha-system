use common_metadata::prelude::MetaExchange;
use pg_mddb::prelude::PostgresExchange;

#[test]
fn test_from_meta_exchange() {
    let meta_exchange = MetaExchange {
        code: "TEST".to_string(),
        name: "Test Exchange".to_string(),
        kaiko_legacy_slug: "test-exchange".to_string(),
    };

    let postgres_exchange = PostgresExchange::from_meta_exchange(meta_exchange);

    assert_eq!(postgres_exchange.exchanges_code, "TEST");
    assert_eq!(postgres_exchange.exchanges_name, "Test Exchange");
}

#[test]
fn test_to_meta_exchange() {
    let postgres_exchange = PostgresExchange {
        exchanges_code: "TEST".to_string(),
        exchanges_name: "Test Exchange".to_string(),
    };

    let meta_exchange = postgres_exchange.to_meta_exchange();

    assert_eq!(meta_exchange.code, "TEST");
    assert_eq!(meta_exchange.name, "Test Exchange");
    assert_eq!(meta_exchange.kaiko_legacy_slug, "");
}
