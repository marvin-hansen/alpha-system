use common_metadata::{MetaExchange, MetaExchangesRoot};

#[test]
fn test_exchanges_root_and_exchange_properties() {
    let exchange = MetaExchange {
        code: "TEST".to_string(),
        name: "Test Exchange".to_string(),
        kaiko_legacy_slug: "test-exchange".to_string(),
    };

    let exchanges_root = MetaExchangesRoot {
        result: "Success".to_string(),
        data: vec![exchange],
    };

    // Test ExchangesRoot properties
    assert_eq!(exchanges_root.result, "Success");
    assert_eq!(exchanges_root.data.len(), 1);

    // Test properties of Exchange
    let test_exchange = &exchanges_root.data[0];
    assert_eq!(test_exchange.code, "TEST");
    assert_eq!(test_exchange.name, "Test Exchange");
    assert_eq!(test_exchange.kaiko_legacy_slug, "test-exchange");
}

#[test]
fn test_exchange_code() {
    let exchange = MetaExchange {
        code: "TEST".to_string(),
        name: "Test Exchange".to_string(),
        kaiko_legacy_slug: "test-exchange".to_string(),
    };

    assert_eq!(exchange.code, "TEST");
}

#[test]
fn test_exchange_name() {
    let exchange = MetaExchange {
        code: "TEST".to_string(),
        name: "Test Exchange".to_string(),
        kaiko_legacy_slug: "test-exchange".to_string(),
    };

    assert_eq!(exchange.name, "Test Exchange");
}

#[test]
fn test_exchange_kaiko_legacy_slug() {
    let exchange = MetaExchange {
        code: "TEST".to_string(),
        name: "Test Exchange".to_string(),
        kaiko_legacy_slug: "test-exchange".to_string(),
    };

    assert_eq!(exchange.kaiko_legacy_slug, "test-exchange");
}
