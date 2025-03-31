/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

// AlphaSocket: An ultra-low latency replacement for Unix domain sockets
// Implementation based on memory-mapped ring buffers for zero-copy IPC

mod listener;
mod ring_buffer;
mod socket;

pub use listener::alpha_listener::AlphaListener;
pub use ring_buffer::{RingBuffer, RingBufferHeader};
pub use socket::alpha_socket::AlphaSocket;
pub use socket::alpha_socket_with_archiver::AlphaSocketWithArchiver;
