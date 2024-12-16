use crate::order_cancel_all::{sbe_decoder, sbe_encoder};
use common_order::OrderCancelAll;
use sbe_types::{SbeDecodeError, SbeEncodeError};

pub trait SbeOrderCancelAllExtension {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError>;
    fn decode_from_sbe(buf: &[u8]) -> Result<OrderCancelAll, SbeDecodeError>;
}

impl SbeOrderCancelAllExtension for OrderCancelAll {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_encoder::encode_order_cancel_all_message(self)
    }

    fn decode_from_sbe(buf: &[u8]) -> Result<OrderCancelAll, SbeDecodeError> {
        sbe_decoder::decode_order_cancel_all_message(buf)
    }
}
