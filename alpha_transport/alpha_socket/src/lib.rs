// AlphaSocket: An ultra-low latency replacement for Unix domain sockets
// Implementation based on memory-mapped ring buffers for zero-copy IPC

mod ring_buffer;
mod socket_listener;
mod socket_stream;

pub use ring_buffer::{RingBuffer, RingBufferHeader};
pub use socket_listener::alpha_listener::AlphaListener;
pub use socket_stream::alpha_stream::AlphaStream;
