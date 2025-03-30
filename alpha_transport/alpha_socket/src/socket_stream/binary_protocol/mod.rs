use crate::AlphaStream;
use binary_protocol::{BinaryClient, BinaryProtocol, Client, ConnectionClient};
use bytes::{Bytes, BytesMut};
use std::io::{Read, Write};
use std::path::Path;
use stream_errors::{ClientError, StreamError};

impl BinaryClient for AlphaStream {}

impl Client for AlphaStream {}

impl ConnectionClient for AlphaStream {
    fn connect<P: AsRef<Path>>(&self, path: P) -> Result<(), ClientError> {
        match AlphaStream::connect(path) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(ClientError::ConnectionError)
            }
        }
    }

    fn shutdown(&self) -> Result<(), ClientError> {
        match AlphaStream::shutdown(self) {
            Ok(()) => Ok(()),
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(ClientError::ShutdownError)
            }
        }
    }
}

impl BinaryProtocol for AlphaStream {
    fn send_all(&mut self, buf: &mut [u8]) -> Result<(), StreamError> {
        match self.write_all(buf) {
            Ok(()) => Ok(()),
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(StreamError::from(ClientError::UnknownError))
            }
        }
    }

    fn read_all(&mut self, buf: &mut [u8]) -> Result<(), StreamError> {
        match self.read_exact(buf) {
            Ok(()) => Ok(()),
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(StreamError::from(ClientError::UnknownError))
            }
        }
    }

    fn try_send_raw_bytes_no_response(&mut self, bytes: &BytesMut) -> Result<(), StreamError> {
        match self.write_with_retry(bytes.as_ref(), 5) {
            Ok(()) => Ok(()),
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(StreamError::from(ClientError::UnknownError))
            }
        }
    }

    fn try_receive_raw_bytes(&mut self, buf: &mut [u8]) -> Result<Option<Bytes>, StreamError> {
        match self.read_with_retry(buf, 5) {
            Ok(n) => {
                if n == 0 {
                    return Ok(None);
                }

                let bytes = Bytes::copy_from_slice(&buf[0..n]);
                Ok(Some(bytes))
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(StreamError::from(ClientError::UnknownError))
            }
        }
    }
}
