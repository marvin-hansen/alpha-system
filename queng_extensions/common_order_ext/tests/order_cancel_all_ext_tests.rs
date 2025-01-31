/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_order::OrderCancelAll;
use common_order_ext::SbeOrderCancelAllExtension;

#[test]
fn test_order_cancel_all_extension() {
    // Full encoding / decoding test suite is in
    // queng_sbe/sbe_messages_order/tests/order_cancel_all
    let order_cancel_all = OrderCancelAll::default();
    let encoded = order_cancel_all.clone().encode_to_sbe().unwrap();
    let decoded = OrderCancelAll::decode_from_sbe(encoded.1.as_slice()).unwrap();
    assert_eq!(order_cancel_all, decoded);
}
