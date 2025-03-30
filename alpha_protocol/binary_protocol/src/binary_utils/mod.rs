use crate::BytesSerializable;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::fmt::Error;

// Binary transmission format:
// - The first 4 bytes are for the payload length. If the payload is empty, the length is 8
// - The next 2 bytes are the command code as U16. See
// - The last 2 bytes are the target host ID as U16. Zero by default for no target host.
// - The remaining bytes are the payload. If the length is 8, then the payload is empty.

/// Length of the initial bytes in the request buffer (4 for length + 4 for command code)
const REQUEST_INITIAL_BYTES_LENGTH: usize = 8usize;

/// Default target host ID is zero (0u16) because the message broker does not use a target host ID
const DEFAULT_TARGET: u16 = 0u16;

/// Builds an immutable byte buffer for sending data over network
///
/// Maximum performance optimization for low latency:
/// - Special fast path for small payloads using stack allocation
/// - Single allocation with exact capacity for larger payloads
/// - Zero redundant calculations or memory operations
/// - Uses direct byte operations for optimal performance
///
/// # Arguments
/// * `command_code` - The command code to include in the request
/// * `b` - The serializable payload to include in the request
///
/// # Returns
/// An immutable `Bytes` buffer containing the serialized request
#[inline]
pub fn pack_raw_bytes(command_code: u16, b: &impl BytesSerializable) -> Bytes {
    // We have to freeze the buffer to make it immutable.
    pack_bytes(command_code, DEFAULT_TARGET, b).freeze()
}

/// Builds an immutable byte buffer for sending data over network
///
/// This is a convenience wrapper around `pack_bytes` that freezes the resulting
/// buffer. It is useful when you want to make sure that the buffer is not modified
/// after it is constructed.
///
/// # Arguments
/// * `command_code` - The command code to include in the request
/// * `target` - The target host ID to include in the request
/// * `b` - The serializable payload to include in the request
///
/// # Returns
/// An immutable `Bytes` buffer containing the serialized request
#[inline]
pub fn pack_raw_bytes_with_target(
    command_code: u16,
    target: u16,
    b: &impl BytesSerializable,
) -> Bytes {
    pack_bytes(command_code, target, b).freeze()
}

/// Builds a raw mutable byte buffer for sending over the network
///
/// Maximum performance optimization for low latency:
/// - Special fast path for small payloads using stack allocation
/// - Single allocation with exact capacity for larger payloads
/// - Zero redundant calculations or memory operations
/// - Uses direct byte operations for optimal performance
///
/// # Arguments
/// * `command_code` - The command code to include in the request
/// * `b` - The serializable payload to include in the request
///
/// # Returns
/// A mutable `Bytes` buffer containing the serialized request
#[inline]
pub fn pack_raw_bytes_mut(command_code: u16, b: &impl BytesSerializable) -> BytesMut {
    pack_bytes(command_code, DEFAULT_TARGET, b)
}

/// Builds a raw mutable byte buffer for sending over the network
///
/// Maximum performance optimization for low latency:
/// - Special fast path for small payloads using stack allocation
/// - Single allocation with exact capacity for larger payloads
/// - Zero redundant calculations or memory operations
/// - Uses direct byte operations for optimal performance
///
/// # Arguments
/// * `command_code` - The command code to include in the request
/// * `target` - The target host ID to include in the request
/// * `b` - The serializable payload to include in the request
///
/// # Returns
/// A mutable `Bytes` buffer containing the serialized request
pub fn pack_raw_bytes_mut_with_target(
    command_code: u16,
    target: u16,
    b: &impl BytesSerializable,
) -> BytesMut {
    pack_bytes(command_code, target, b)
}

/// Builds a raw mutable byte buffer for sending over the network
///
/// Maximum performance optimization for low latency:
/// - Special fast path for small payloads using stack allocation
/// - Single allocation with exact capacity for larger payloads
/// - Zero redundant calculations or memory operations
/// - Uses direct byte operations for optimal performance
///
/// # Arguments
/// * `command_code` - The command code to include in the request
/// * `target` - The target host ID to include in the request
/// * `b` - The serializable payload to include in the request
///
/// # Returns
/// A mutable `Bytes` buffer containing the serialized request
///
#[inline]
fn pack_bytes(command_code: u16, target: u16, b: &impl BytesSerializable) -> BytesMut {
    // Get the serialized payload;
    // Note, error propagation would take a 18% - 25% performance hit;
    // thus expect() instead of match.
    let payload = b.to_bytes().expect("Failed to serialize payload");
    let payload_len = payload.len();

    // Fast path for empty payloads
    if payload_len == 0 {
        let mut buf = BytesMut::with_capacity(REQUEST_INITIAL_BYTES_LENGTH);

        buf.put_u32_le(REQUEST_INITIAL_BYTES_LENGTH as u32);
        buf.put_u16_le(command_code);
        buf.put_u16_le(target);
        return buf;
    }

    // Calculate total length once
    let total_len = REQUEST_INITIAL_BYTES_LENGTH + payload_len;

    // Allocate buffer with exact capacity
    let mut mut_buffer = BytesMut::with_capacity(total_len);

    // Write length prefix (4 bytes for length + 4 bytes for command + payload)
    mut_buffer.put_u32_le(total_len as u32);

    // Write command code
    mut_buffer.put_u16_le(command_code);

    // Write target host ID
    mut_buffer.put_u16_le(target);

    // Extend with payload (zero-copy where possible)
    mut_buffer.extend_from_slice(&payload);

    // Return the mut buffer
    mut_buffer
}

/// Unpacks a raw byte buffer received from the network
///
/// # Safety
/// This function assumes that the input buffer is a valid serialized
/// request. If the input buffer is invalid, this function will return an
/// error.
///
/// # Arguments
/// * `b` - The raw byte buffer received from the network
///
/// # Returns
/// A tuple containing the length of the payload, the command code,
/// the target host ID, and the payload itself
///
pub fn unpack_raw_bytes(b: &[u8]) -> Result<(u32, u16, u16, Bytes), Error> {
    // Ensure that the buffer has at least 8 bytes
    if b.len() < REQUEST_INITIAL_BYTES_LENGTH {
        return Err(Error);
    }

    // The first 4 bytes are for the payload length
    let payload_len = (&b[0..4]).get_u32_le();

    // The next 2 bytes are the command code as U16
    let command_code = (&b[4..6]).get_u16_le();

    // The last 2 bytes are the target as U16
    let target_client = (&b[6..8]).get_u16_le();

    // Extract the payload based on its length
    // If the payload length is equal to the REQUEST_INITIAL_BYTES_LENGTH, then
    // the subsequent payload id empty so you can return early.

    // Fast path for empty payload since it happens often i.e for all void, ack, ping, or heartbeat requests
    if payload_len as usize == REQUEST_INITIAL_BYTES_LENGTH {
        // Empty payload
        return Ok((payload_len, command_code, target_client, Bytes::new()));
    }

    // Extract the payload based on its length
    let payload = if payload_len as usize > REQUEST_INITIAL_BYTES_LENGTH {
        // Non-empty payload, extract it
        Bytes::copy_from_slice(&b[REQUEST_INITIAL_BYTES_LENGTH..])
    } else {
        // Invalid payload length
        return Err(Error);
    };

    Ok((payload_len, command_code, target_client, payload))
}
