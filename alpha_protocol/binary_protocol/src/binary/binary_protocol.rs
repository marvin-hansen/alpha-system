use bytes::{Bytes, BytesMut};
use stream_errors::StreamError;

pub trait BinaryProtocol {
    /// Send all the data in the buffer without blocking.
    ///
    /// This method sends all the data in the buffer without blocking. If any
    /// errors occur, a StreamError is returned.
    ///
    /// # Returns
    ///
    /// A Result containing an error if the send failed. If the send is
    /// successful, the method returns Ok(()).
    ///
    fn send_all(&mut self, buf: &mut [u8]) -> Result<(), StreamError>;

    /// Read all the data from the connection and write it to the buffer.
    ///
    /// This method reads all the data from the connection and writes it to the
    /// buffer. If any errors occur, a StreamError is returned.
    ///
    /// # Returns
    ///
    /// A Result containing an error if the read failed. If the read is
    /// successful, the method returns Ok(()).
    ///
    fn read_all(&mut self, buf: &mut [u8]) -> Result<(), StreamError>;

    /// Try to send raw bytes without blocking
    ///
    /// This method attempts to send data but returns a StreamError if the
    /// send fails instead of waiting.
    ///
    /// # Returns
    ///
    /// A Result containing an error if the send failed. If the send is
    /// successful, the method returns Ok(())
    ///
    fn try_send_raw_bytes_no_response(&mut self, bytes: &BytesMut) -> Result<(), StreamError>;

    /// Try to receive raw bytes without blocking
    ///
    /// This method attempts to receive data but returns None if no data
    /// is immediately available instead of waiting.
    ///
    /// # Returns
    ///
    /// A Result containing an Option with the received bytes or an error
    fn try_receive_raw_bytes(&mut self, buf: &mut [u8]) -> Result<Option<Bytes>, StreamError>;
}
