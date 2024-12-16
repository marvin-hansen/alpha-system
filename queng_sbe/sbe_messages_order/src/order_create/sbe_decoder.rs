use common_order::OrderCreate;
use sbe_bindings::order_create_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{MessageHeaderDecoder, OrderCreateDecoder, ReadBuf};
use sbe_types::SbeDecodeError;

pub(crate) fn decode_order_create_message(buffer: &[u8]) -> Result<OrderCreate, SbeDecodeError> {
    let mut csg = OrderCreateDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    // let exchange_id = ExchangeID::from(csg.exchange_id());
    //
    // let client_id = csg.client_id();

    Err(SbeDecodeError("Not implemented".to_string()))
}
