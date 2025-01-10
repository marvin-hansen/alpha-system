use common_exchange::ExchangeID;
use common_order::{OrderSide, OrderType, OrderUpdate, TimeInForce};
use rust_decimal::Decimal;

#[test]
fn test_order_update_extension() {
    let _order = OrderUpdate::new(
        ExchangeID::BinanceSpot,
        1, // client_order_id
        "cl_ord_id".to_string().into(),
        "exchange_oder_id".into(),
        "BTCUSD".into(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(15, 1),
        Decimal::new(500000000, 1),
    );

    // Full encoding / decoding test suite is in
    // queng_sbe/sbe_messages_order/tests/order_update

    // let (_, encoded) = order.clone().encode_to_sbe().unwrap();
    // let decoded = OrderUpdate::decode_from_sbe(encoded.as_slice()).unwrap();
    // assert_eq!(order.client_id(), decoded.client_id());
}
