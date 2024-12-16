use crate::order_cancel::{sbe_decoder, sbe_encoder};
use common_order::OrderCancel;
use sbe_types::{SbeDecodeError, SbeEncodeError};

pub trait SbeOrderCancelExtension {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError>;

    fn decode_from_sbe(buf: &[u8]) -> Result<OrderCancel, SbeDecodeError>;
}

impl SbeOrderCancelExtension for OrderCancel {
    fn encode_to_sbe(self) -> Result<(usize, Vec<u8>), SbeEncodeError> {
        sbe_encoder::encode_order_cancel_message(self)
    }

    fn decode_from_sbe(buf: &[u8]) -> Result<OrderCancel, SbeDecodeError> {
        sbe_decoder::decode_order_cancel_message(buf)
    }
}
