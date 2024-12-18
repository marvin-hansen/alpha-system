use common_exchange::ExchangeID;
use common_order::{OrderCreate, OrderSide, OrderType, TimeInForce};
use common_order_ext::SbeOrderCreateExtension;
use rust_decimal::Decimal;

#[test]
fn test_order_create_extension() {
    let order = OrderCreate::new(
        ExchangeID::Binance,
        1, // client_order_id
        "cl_ord_id".to_string().into(),
        "BTCUSD".to_string(),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(15, 1),
        Decimal::new(500000000, 1),
    );

    // Full encoding / decoding test suite is in
    // queng_sbe/sbe_messages_order/tests/order_create

    let encoded = order.clone().encode_to_sbe().unwrap();
    let decoded = OrderCreate::decode_from_sbe(encoded.1.as_slice()).unwrap();
    assert_eq!(order.client_id(), decoded.client_id());
}
