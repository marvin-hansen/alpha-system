/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use proto_mddb::proto::ProtoMetaExchange;

#[test]
fn test_proto_meta_exchange_request() {
    let exchange = ProtoMetaExchange {
        exchange_code: "exchange_code".to_string(),
        exchange_name: "exchange_name".to_string(),
        exchange_hash: "exchange_hash".to_string(),
    };

    assert_eq!(exchange.exchange_code, "exchange_code");
    assert_eq!(exchange.exchange_name, "exchange_name");
    assert_eq!(exchange.exchange_hash, "exchange_hash");
}
