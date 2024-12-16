use common_exchange::ExchangeID;
use common_order::OrderCancelAll;
use sbe_bindings::order_cancel_all_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{MessageHeaderDecoder, OrderCancelAllDecoder, ReadBuf};

use sbe_types::SbeDecodeError;

pub(crate) fn decode_order_cancel_all_message(
    buffer: &[u8],
) -> Result<OrderCancelAll, SbeDecodeError> {
    let mut csg = OrderCancelAllDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let exchange_id = ExchangeID::from(csg.exchange_id());

    let client_id = csg.client_id();

    Ok(OrderCancelAll::new(exchange_id, client_id))
}
