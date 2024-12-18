use common_exchange::ExchangeID;
use common_order::{
    ClientOrderID, OrderCreate, OrderExchangeSymbol, OrderSide, OrderType, TimeInForce,
};
use common_order_ext::SbeOrderCreateExtension;
use rust_decimal::Decimal;

#[test]
fn test_order_create_extension() {
    let order = OrderCreate::new(
        ExchangeID::Binance,
        1, // client_order_id
        ClientOrderID::new("cl_ord_id"),
        OrderExchangeSymbol::new("BTCUSD"),
        OrderType::Limit,
        OrderSide::Buy,
        TimeInForce::GoodTillCancel,
        None,
        Decimal::new(15, 1),
        Decimal::new(500000000, 1),
    );

    // Full encoding / decoding test suite is in
    // queng_sbe/sbe_messages_order/tests/order_create

    let result = order.clone().encode_to_sbe();
    assert!(result.is_ok());

    let (_, encoded) = result.unwrap();

    let result = OrderCreate::decode_from_sbe(encoded.as_slice());
    assert!(result.is_ok());

    let decoded = result.unwrap();
    assert_eq!(order.client_id(), decoded.client_id());
}
