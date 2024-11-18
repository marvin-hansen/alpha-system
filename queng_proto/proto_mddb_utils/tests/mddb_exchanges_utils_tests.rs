use common_metadata::MetaExchange;
use proto_mddb::proto::ProtoMetaExchange;
use proto_mddb_utils::*;

#[test]
fn test_get_check_if_exchange_exists_request() {
    let exchange_code = "binance";
    let request = get_check_if_exchange_exists_request(exchange_code);
    assert_eq!(request.exchange_code, exchange_code);
}

#[test]
fn test_get_exchange_request() {
    let exchange_code = "kraken";
    let request = get_exchange_request(exchange_code);
    assert_eq!(request.exchange_code, exchange_code);
}

#[test]
fn test_get_all_exchanges_request() {
    let request = get_all_exchanges_request();
    assert_eq!(format!("{:?}", request), "GetAllExchangesRequest");
}

#[test]
fn test_get_lookup_exchange_name_request() {
    let exchange_code = "coinbase";
    let request = get_lookup_exchange_name_request(exchange_code);
    assert_eq!(request.exchange_code, exchange_code);
}

#[test]
fn test_get_count_exchanges_response() {
    let count = 42;
    let response = get_count_exchanges_response(count);
    assert_eq!(response.count, count);
}

#[test]
fn test_get_check_if_exchange_exists_response() {
    let exists = true;
    let response = get_check_if_exchange_exists_response(exists);
    assert_eq!(response.exists, exists);
}

#[test]
fn test_get_exchange_response() {
    let meta_exchange = MetaExchange {
        code: "bitfinex".to_string(),
        name: "Bitfinex".to_string(),
        kaiko_legacy_slug: "".to_string(),
    };
    let response = get_exchange_response(Some(meta_exchange.clone()));
    let proto_exchange = response.exchange.unwrap();
    assert_eq!(proto_exchange.exchange_code, meta_exchange.code);
    assert_eq!(proto_exchange.exchange_name, meta_exchange.name);
    assert_eq!(proto_exchange.exchange_hash, meta_exchange.hash());
}

#[test]
fn test_get_all_exchanges_response() {
    let meta_exchanges = vec![
        MetaExchange {
            code: "huobi".to_string(),
            name: "Huobi".to_string(),
            kaiko_legacy_slug: "".to_string(),
        },
        MetaExchange {
            code: "okex".to_string(),
            name: "OKEx".to_string(),
            kaiko_legacy_slug: "".to_string(),
        },
    ];

    let response = get_all_exchanges_response(meta_exchanges.clone());
    assert_eq!(response.exchanges.len(), 2);

    for (proto_exchange, meta_exchange) in response.exchanges.iter().zip(meta_exchanges.iter()) {
        assert_eq!(proto_exchange.exchange_code, meta_exchange.code);
        assert_eq!(proto_exchange.exchange_name, meta_exchange.name);
        assert_eq!(proto_exchange.exchange_hash, meta_exchange.hash());
    }
}

#[test]
fn test_meta_exchange_to_proto_exchange() {
    let meta_exchange = MetaExchange {
        code: "kucoin".to_string(),
        name: "KuCoin".to_string(),
        kaiko_legacy_slug: "".to_string(),
    };

    let proto_exchange = meta_exchange_to_proto_exchange(&meta_exchange);
    assert_eq!(proto_exchange.exchange_code, meta_exchange.code);
    assert_eq!(proto_exchange.exchange_name, meta_exchange.name);
    assert_eq!(proto_exchange.exchange_hash, meta_exchange.hash());
}

#[test]
fn test_proto_exchange_to_meta_exchange() {
    let proto_exchange = ProtoMetaExchange {
        exchange_code: "bitstamp".to_string(),
        exchange_name: "Bitstamp".to_string(),
        exchange_hash: "some_hash".to_string(),
    };

    let meta_exchange = proto_exchange_to_meta_exchange(&proto_exchange);
    assert_eq!(meta_exchange.code, proto_exchange.exchange_code);
    assert_eq!(meta_exchange.name, proto_exchange.exchange_name);
    assert_eq!(meta_exchange.kaiko_legacy_slug, "");
}
