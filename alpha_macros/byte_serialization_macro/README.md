# Byte Serialization Macro

A procedural macro for automatically implementing the `BytesSerializable` trait for structs.

## Overview

This crate provides a derive macro `BytesSerialization` that automatically implements the `BytesSerializable` trait for structs. The trait provides methods for serializing a struct to bytes and deserializing bytes back to a struct.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
byte_serialization_macro = {path = "delta_macros/byte_serialization_macro"}
```

Then, you can use the derive macro in your code:

```rust
use rkyv::{Archive, Deserialize, Serialize};
use byte_serialization_macro::BytesSerialization;

#[derive(Archive, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
#[derive(BytesSerialization)]
pub struct DeleteTopic {
    /// Unique stream ID (numeric or name).
    stream_id: u32, // 4 bytes
    /// Unique topic ID (numeric), if None is provided then the server will automatically assign it.
    topic_id: u32, // 4 bytes
}


// The macro automatically implements the BytesSerializable trait:
// impl BytesSerializable for DeleteTopic {
//     fn to_bytes(&self) -> Result<AlignedVec, StreamError> { ... }
//     fn from_bytes(bytes: &[u8]) -> Result<Self, StreamError> { ... }
// }

// Example usage:
fn main() -> Result<(), StreamError> {
    let topic = DeleteTopic {
        stream_id: 1,
        topic_id: 2,
    };
    
    // Serialize to bytes
    let bytes = topic.to_bytes()?;
    
    // Deserialize from bytes
    let deserialized_topic = DeleteTopic::from_bytes(&bytes)?;
    
    assert_eq!(topic.stream_id, deserialized_topic.stream_id);
    assert_eq!(topic.topic_id, deserialized_topic.topic_id);
    
    Ok(())
}
```

## Requirements

For the macro to work properly, the following must be available in the scope where the derived struct is defined:

- The `BytesSerializable` trait
- The `StreamError` type
- The `Sizeable` trait (for the `byte_size()` method)
- The `rkyv` crate with its serialization/deserialization functionality

## Implementation Details

The macro generates code that:

1. Uses `rkyv::api::high::to_bytes_with_alloc` for serialization
2. Uses `rkyv::from_bytes` for deserialization
3. Properly handles errors with appropriate `StreamError` variants
4. Uses the `byte_size()` method from the `Sizeable` trait to determine the capacity needed for serialization

## License

This crate is licensed under the same terms as the Deltastream project.
