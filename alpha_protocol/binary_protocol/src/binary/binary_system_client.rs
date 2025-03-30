use crate::binary::binary_protocol::BinaryProtocol;

use crate::{PING_CODE, Ping, binary_utils};
use stream_errors::StreamError;

pub trait SystemClient {
    fn ping(&mut self) -> Result<(), StreamError>;
}

impl<B: BinaryProtocol> SystemClient for B {
    fn ping(&mut self) -> Result<(), StreamError> {
        let bytes = binary_utils::pack_raw_bytes_mut(PING_CODE, &Ping {});
        self.try_send_raw_bytes_no_response(&bytes)?;

        Ok(())
    }
}
