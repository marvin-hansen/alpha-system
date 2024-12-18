use common_order::OrderCancel;
use common_order_ext::SbeOrderCancelExtension;

#[test]
fn test_order_cancel_extension() {
    // Full encoding / decoding test suite is in
    // queng_sbe/sbe_messages_order/tests/order_cancel
    let order_cancel = OrderCancel::default();
    let encoded = order_cancel.clone().encode_to_sbe().unwrap();
    let decoded = OrderCancel::decode_from_sbe(encoded.1.as_slice()).unwrap();
    assert_eq!(order_cancel, decoded);
}
