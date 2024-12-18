use common_exchange::ExchangeID;
use common_order::OrderUpdate;
use sbe_bindings::order_update_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{
    message_header_codec::MessageHeaderDecoder, order_update_codec::OrderUpdateDecoder, ReadBuf,
};
use sbe_types::SbeDecodeError;

pub fn decode_order_update_message(buffer: &[u8]) -> Result<OrderUpdate, SbeDecodeError> {
    let mut csg = OrderUpdateDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header, 0);

    let _exchange_id = ExchangeID::from(csg.exchange_id());

    // let client_id = csg.client_id();

    Ok(OrderUpdate::default())
}
