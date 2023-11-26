use sbe_bindings::{message_header_codec, Encoder, SbeResult, StartDataMsgEncoder, WriteBuf};
use sbe_bindings::{ExchangeID as SbeExchangeID, MessageType as SbeMessageType};

use crate::prelude::StartDataMessage;
use crate::utils::sbe_encode_utils;

impl StartDataMessage {
    pub fn encode(&self) -> SbeResult<(usize, Vec<u8>)> {
        let mut buffer = vec![0u8; 256];

        let mut csg = StartDataMsgEncoder::default();

        csg = csg.wrap(
            WriteBuf::new(buffer.as_mut_slice()),
            message_header_codec::ENCODED_LENGTH,
        );
        csg = csg.header(0).parent().expect("Failed to encode header");

        let value = SbeMessageType::from(self.message_type as u8);
        csg.message_type(value);

        let value = SbeExchangeID::from(self.exchange_id as u8);
        csg.exchange_id(value);

        let value = sbe_encode_utils::encode_string(&self.asset);
        csg.asset(value);

        let limit = csg.get_limit();
        Ok((limit, buffer))
    }
}
