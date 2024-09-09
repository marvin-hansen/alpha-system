use common_metadata::prelude::MetaExchange;
use pg_metadb::prelude::*;

#[test]
fn test_from_meta_exchange() {
    let meta_exchange = MetaExchange {
        code: "TEST".to_string(),
        name: "Test Exchange".to_string(),
        kaiko_legacy_slug: "test-exchange".to_string(),
    };

    let postgres_exchange = Exchange::from_meta_exchange(meta_exchange);

    assert_eq!(postgres_exchange.exchange_id, "TEST");
    assert_eq!(postgres_exchange.exchange_name, "Test Exchange");
}

#[test]
fn test_to_meta_exchange() {
    let postgres_exchange = Exchange {
        exchange_id: "TEST".to_string(),
        exchange_name: "Test Exchange".to_string(),
    };

    let meta_exchange = postgres_exchange.to_meta_exchange();

    assert_eq!(meta_exchange.code, "TEST");
    assert_eq!(meta_exchange.name, "Test Exchange");
    assert_eq!(meta_exchange.kaiko_legacy_slug, "");
}
