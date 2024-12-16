use crate::order_create::{sbe_decoder, sbe_encoder};
use common_order::OrderCreate;
use sbe_types::{SbeDecodeError, SbeEncodeError};

pub trait SbeOrderCreateExtension {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError>;
    fn decode_from_sbe(data: &[u8]) -> Result<OrderCreate, SbeDecodeError>;
}

impl SbeOrderCreateExtension for OrderCreate {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_encoder::encode_order_create_message(self)
    }

    fn decode_from_sbe(data: &[u8]) -> Result<OrderCreate, SbeDecodeError> {
        sbe_decoder::decode_order_create_message(data)
    }
}
