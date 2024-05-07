use common::prelude::{Exchange, ExchangesRoot};

#[test]
fn test_exchanges_root_and_exchange_properties() {
    let exchange = Exchange {
        code: "TEST".to_string(),
        name: "Test Exchange".to_string(),
        kaiko_legacy_slug: "test-exchange".to_string(),
        url: Some("http://www.testexchange.com".to_string()),
    };

    let exchanges_root = ExchangesRoot {
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
    assert_eq!(
        test_exchange.url,
        Some("http://www.testexchange.com".to_string())
    );
}
