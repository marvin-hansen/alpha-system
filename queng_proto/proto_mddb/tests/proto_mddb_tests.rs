use proto_mddb::proto::*;

#[test]
fn test_lookup_exchange_name_request() {
    let request = LookupExchangeNameRequest { exchange_id: 1 };
    assert_eq!(request.exchange_id, 1);
}

#[test]
fn test_lookup_symbol_request() {
    let request = LookupSymbolRequest {
        exchange_id: 1,
        symbol_id: 1,
    };
    assert_eq!(request.exchange_id, 1);
    assert_eq!(request.symbol_id, 1);
}

#[test]
fn test_lookup_symbol_id_request() {
    let request = LookupSymbolIdRequest {
        exchange_id: 1,
        symbol: String::from("test"),
    };
    assert_eq!(request.exchange_id, 1);
    assert_eq!(request.symbol, String::from("test"));
}

#[test]
fn test_lookup_exchange_name_response() {
    let response = LookupExchangeNameResponse {
        exchange_name: String::from("test"),
    };
    assert_eq!(response.exchange_name, String::from("test"));
}

#[test]
fn test_lookup_symbol_response() {
    let response = LookupSymbolResponse {
        exchange_name: String::from("test"),
        symbol: String::from("test"),
    };

    assert_eq!(response.exchange_name, String::from("test"));
    assert_eq!(response.symbol, String::from("test"));
}

#[test]
fn test_lookup_symbol_id_response() {
    let response = LookupSymbolIdResponse {
        exchange_name: String::from("test"),
        symbol_id: 1,
    };

    assert_eq!(response.exchange_name, String::from("test"));
    assert_eq!(response.symbol_id, 1);
}
