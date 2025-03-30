# Prompt

Please design and implement one macros in 
the byte_serialization_macro crate:

BytesSerialization

Please
1. Design the BytesSerialization macro based on the requirements below:
2. Implement the BytesSerialization macro 
3. Build the byte_serialization_macro crate using cargo build -p byte_serialization_macro to ensure it compiles.

Requirements:

For a given struct, the macro generates the entire BytesSerializable trait.

The BytesSerializable trait is defined as follows:

```rust
pub trait BytesSerializable {
    fn to_bytes(&self) -> Result<AlignedVec, StreamError>;

    fn from_bytes(bytes: &[u8]) -> Result<Self, StreamError>
    where
        Self: Sized;
}
```


For example, the DeleteTopic struct looks as follows

```rust
pub struct DeleteTopic {
    stream_id: u32, // 4 bytes
    topic_id: u32, // 4 bytes
}
``` 

Adding the BytesSerialization macro like so:

```rust
#[derive(BytesSerialization)] 
pub struct DeleteTopic {
    stream_id: u32, // 4 bytes
    topic_id: u32, // 4 bytes
}
``` 

Then generates the following trait implementation:

```rust
use rkyv::api::high::to_bytes_with_alloc;
use rkyv::rancor::Error;
use rkyv::ser::allocator::Arena;
use rkyv::util::AlignedVec;
use crate::{BytesSerializable, CreateTopic, Sizeable, StreamError};

impl BytesSerializable for CreateTopic {
    fn to_bytes(&self) -> Result<AlignedVec, StreamError> {
        let capacity = self.byte_size();
        let mut arena = Arena::with_capacity(capacity);
        match to_bytes_with_alloc::<_, Error>(self, arena.acquire()) {
            Ok(bytes) => Ok(bytes),
            Err(error) => Err(StreamError::SerializationError(error.to_string())),
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, StreamError> {
        let deserialized: Self = match rkyv::from_bytes::<Self, Error>(&bytes) {
            Ok(deserialized) => deserialized,
            Err(error) => return Err(StreamError::DeserializationError(error.to_string())),
        };
        Ok(deserialized)
    }
}
```  

Please
1. Design the macro based on the requirements below:
2. Implement the macro
3. Build the byte_serialization_macro crate using cargo build -p byte_serialization_macro to ensure it compiles.
