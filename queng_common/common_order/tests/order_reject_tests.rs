use common_order::OrderRejectReason;

#[test]
fn test_order_reject_reason_conversion_from_u8() {
    // Test all defined conversions
    assert_eq!(OrderRejectReason::from(0), OrderRejectReason::OTHER);
    assert_eq!(OrderRejectReason::from(1), OrderRejectReason::UnknownSymbol);
    assert_eq!(
        OrderRejectReason::from(2),
        OrderRejectReason::UnknownExchange
    );
    assert_eq!(
        OrderRejectReason::from(3),
        OrderRejectReason::ExchangeClosed
    );
    assert_eq!(
        OrderRejectReason::from(4),
        OrderRejectReason::ExchangeUnreachable
    );
    assert_eq!(
        OrderRejectReason::from(5),
        OrderRejectReason::ExchangeResponseTimeout
    );
    assert_eq!(
        OrderRejectReason::from(6),
        OrderRejectReason::InvalidOrderType
    );
    assert_eq!(
        OrderRejectReason::from(7),
        OrderRejectReason::InvalidTimeInForce
    );
    assert_eq!(
        OrderRejectReason::from(8),
        OrderRejectReason::InvalidQuantity
    );
    assert_eq!(OrderRejectReason::from(9), OrderRejectReason::InvalidPrice);
    assert_eq!(
        OrderRejectReason::from(10),
        OrderRejectReason::InvalidStopPrice
    );
    assert_eq!(
        OrderRejectReason::from(11),
        OrderRejectReason::InvalidLossPrice
    );
    assert_eq!(
        OrderRejectReason::from(12),
        OrderRejectReason::InvalidOrderId
    );
    assert_eq!(
        OrderRejectReason::from(13),
        OrderRejectReason::OrderIdNotFound
    );
    assert_eq!(
        OrderRejectReason::from(14),
        OrderRejectReason::OrderStatusConflict
    );

    // Test fallback for unknown values
    assert_eq!(OrderRejectReason::from(42), OrderRejectReason::OTHER);
    assert_eq!(OrderRejectReason::from(255), OrderRejectReason::OTHER);
}

#[test]
fn test_order_reject_reason_conversion_to_u8() {
    // Test conversion from OrderRejectReason to u8
    assert_eq!(u8::from(OrderRejectReason::OTHER), 0);
    assert_eq!(u8::from(OrderRejectReason::UnknownSymbol), 1);
    assert_eq!(u8::from(OrderRejectReason::UnknownExchange), 2);
    assert_eq!(u8::from(OrderRejectReason::ExchangeClosed), 3);
    assert_eq!(u8::from(OrderRejectReason::ExchangeUnreachable), 4);
    assert_eq!(u8::from(OrderRejectReason::ExchangeResponseTimeout), 5);
    assert_eq!(u8::from(OrderRejectReason::InvalidOrderType), 6);
    assert_eq!(u8::from(OrderRejectReason::InvalidTimeInForce), 7);
    assert_eq!(u8::from(OrderRejectReason::InvalidQuantity), 8);
    assert_eq!(u8::from(OrderRejectReason::InvalidPrice), 9);
    assert_eq!(u8::from(OrderRejectReason::InvalidStopPrice), 10);
    assert_eq!(u8::from(OrderRejectReason::InvalidLossPrice), 11);
    assert_eq!(u8::from(OrderRejectReason::InvalidOrderId), 12);
    assert_eq!(u8::from(OrderRejectReason::OrderIdNotFound), 13);
    assert_eq!(u8::from(OrderRejectReason::OrderStatusConflict), 14);
}

#[test]
fn test_order_reject_reason_default() {
    assert_eq!(OrderRejectReason::default(), OrderRejectReason::OTHER);
}

#[test]
fn test_order_reject_reason_display() {
    assert_eq!(OrderRejectReason::OTHER.to_string(), "OTHER");
    assert_eq!(
        OrderRejectReason::UnknownSymbol.to_string(),
        "UnknownSymbol"
    );
    assert_eq!(
        OrderRejectReason::UnknownExchange.to_string(),
        "UnknownExchange"
    );
    assert_eq!(
        OrderRejectReason::ExchangeClosed.to_string(),
        "ExchangeClosed"
    );
    assert_eq!(
        OrderRejectReason::ExchangeUnreachable.to_string(),
        "ExchangeUnreachable"
    );
    assert_eq!(
        OrderRejectReason::ExchangeResponseTimeout.to_string(),
        "ExchangeResponseTimeout"
    );
    assert_eq!(
        OrderRejectReason::InvalidOrderType.to_string(),
        "InvalidOrderType"
    );
    assert_eq!(
        OrderRejectReason::InvalidTimeInForce.to_string(),
        "InvalidTimeInForce"
    );
    assert_eq!(
        OrderRejectReason::InvalidQuantity.to_string(),
        "InvalidQuantity"
    );
    assert_eq!(OrderRejectReason::InvalidPrice.to_string(), "InvalidPrice");
    assert_eq!(
        OrderRejectReason::InvalidStopPrice.to_string(),
        "InvalidStopPrice"
    );
    assert_eq!(
        OrderRejectReason::InvalidLossPrice.to_string(),
        "InvalidLossPrice"
    );
    assert_eq!(
        OrderRejectReason::InvalidOrderId.to_string(),
        "InvalidOrderId"
    );
    assert_eq!(
        OrderRejectReason::OrderIdNotFound.to_string(),
        "OrderIdNotFound"
    );
    assert_eq!(
        OrderRejectReason::OrderStatusConflict.to_string(),
        "OrderStatusConflict"
    );
}

#[test]
fn test_order_reject_reason_debug() {
    assert_eq!(format!("{:?}", OrderRejectReason::OTHER), "OTHER");
    assert_eq!(
        format!("{:?}", OrderRejectReason::UnknownSymbol),
        "UnknownSymbol"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::UnknownExchange),
        "UnknownExchange"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::ExchangeClosed),
        "ExchangeClosed"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::ExchangeUnreachable),
        "ExchangeUnreachable"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::ExchangeResponseTimeout),
        "ExchangeResponseTimeout"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::InvalidOrderType),
        "InvalidOrderType"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::InvalidTimeInForce),
        "InvalidTimeInForce"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::InvalidQuantity),
        "InvalidQuantity"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::InvalidPrice),
        "InvalidPrice"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::InvalidStopPrice),
        "InvalidStopPrice"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::InvalidLossPrice),
        "InvalidLossPrice"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::InvalidOrderId),
        "InvalidOrderId"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::OrderIdNotFound),
        "OrderIdNotFound"
    );
    assert_eq!(
        format!("{:?}", OrderRejectReason::OrderStatusConflict),
        "OrderStatusConflict"
    );
}

#[test]
fn test_order_reject_reason_clone_and_eq() {
    let original = OrderRejectReason::UnknownSymbol;
    let cloned = original.clone();
    assert_eq!(original, cloned);

    // Test inequality
    assert_ne!(original, OrderRejectReason::UnknownExchange);
    assert_ne!(original, OrderRejectReason::OTHER);

    // Test all unique variants
    let variants = [
        OrderRejectReason::OTHER,
        OrderRejectReason::UnknownSymbol,
        OrderRejectReason::UnknownExchange,
        OrderRejectReason::ExchangeClosed,
        OrderRejectReason::ExchangeUnreachable,
        OrderRejectReason::ExchangeResponseTimeout,
        OrderRejectReason::InvalidOrderType,
        OrderRejectReason::InvalidTimeInForce,
        OrderRejectReason::InvalidQuantity,
        OrderRejectReason::InvalidPrice,
        OrderRejectReason::InvalidStopPrice,
        OrderRejectReason::InvalidLossPrice,
        OrderRejectReason::InvalidOrderId,
        OrderRejectReason::OrderIdNotFound,
        OrderRejectReason::OrderStatusConflict,
    ];

    // Ensure all variants are unique
    for (i, variant1) in variants.iter().enumerate() {
        for variant2 in variants.iter().skip(i + 1) {
            assert_ne!(variant1, variant2);
        }
    }
}
