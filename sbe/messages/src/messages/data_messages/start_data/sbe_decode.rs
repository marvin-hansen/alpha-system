use crate::prelude::{MessageType, StartDataMessage};
use common::prelude::ExchangeID;
use sbe_bindings::{MessageHeaderDecoder, ReadBuf, SbeResult, StartDataMsgDecoder};

use sbe_bindings::start_data_msg_codec::SBE_TEMPLATE_ID;

pub fn decode_start_data_message(buffer: &[u8]) -> SbeResult<StartDataMessage> {
    let mut csg = StartDataMsgDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type =
        MessageType::try_from(sbe_message_type as u8).expect("Failed to convert message type");

    let sbe_exchange_id = csg.exchange_id();
    let exchange_id =
        ExchangeID::try_from(sbe_exchange_id as u8).expect("Failed to convert exchange id");

    let sbe_asset = csg.asset();
    let asset = String::from_utf8(Vec::from(sbe_asset)).expect("Failed to convert asset");

    let start_data_message = StartDataMessage {
        message_type,
        exchange_id,
        asset,
    };

    Ok(start_data_message)
}
